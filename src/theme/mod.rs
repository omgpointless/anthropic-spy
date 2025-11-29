// Theme system for the TUI
//
// Architecture:
// 1. ColorPalette: 16-color ANSI palette + bg/fg/cursor/selection (VHS-compatible)
// 2. SemanticTheme: Maps palette colors to UI element meanings
// 3. Theme: Final resolved theme with all colors ready for use
//
// Theme loading priority:
// 1. External themes from ~/.config/anthropic-spy/themes/*.json
// 2. Embedded themes compiled into binary
// 3. Fallback to "One Half Dark" (default)

mod embedded;
mod palette;
mod semantic;

pub use embedded::list_embedded_themes;
pub use palette::ColorPalette;
pub use semantic::SemanticTheme;

use ratatui::style::Color;
use std::path::PathBuf;

/// Theme configuration options
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    /// Use theme's background color (true) or terminal's default (false)
    pub use_theme_background: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            use_theme_background: true, // Opinionated: use theme bg by default
        }
    }
}

/// Complete resolved theme ready for use in the TUI.
/// This maintains the same public API as the original Theme struct.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,

    // ─── Event Type Colors ───────────────────────────────────
    pub tool_call: Color,
    pub tool_result_ok: Color,
    pub tool_result_fail: Color,
    pub request: Color,
    pub response: Color,
    pub error: Color,
    pub thinking: Color,
    pub api_usage: Color,
    pub headers: Color,
    pub rate_limit: Color,
    pub context_compact: Color,

    // ─── UI Element Colors ───────────────────────────────────
    pub context_bar_fill: Color,
    pub context_bar_warn: Color,
    pub context_bar_danger: Color,
    pub status_bar: Color,
    pub title: Color,
    pub border: Color,
    pub highlight: Color,

    // ─── Panel Identity Colors ───────────────────────────────
    pub panel_events: Color,
    pub panel_thinking: Color,
    pub panel_logs: Color,
    pub panel_detail: Color,

    // ─── Terminal Colors (new) ───────────────────────────────
    pub background: Color,
    pub foreground: Color,

    // ─── Source palette (for VHS export) ─────────────────────
    palette: ColorPalette,
}

impl Theme {
    /// Load theme by name with default configuration
    pub fn by_name(name: &str) -> Self {
        Self::by_name_with_config(name, &ThemeConfig::default())
    }

    /// Load theme by name with custom configuration
    pub fn by_name_with_config(name: &str, config: &ThemeConfig) -> Self {
        // Try loading from various sources
        let palette = Self::load_palette(name);
        Self::from_palette_with_config(palette, config)
    }

    /// Load palette from embedded themes or external files
    fn load_palette(name: &str) -> ColorPalette {
        // 1. Try embedded themes first
        if let Some(json) = embedded::get_embedded_theme(name) {
            if let Ok(palette) = ColorPalette::from_json(json) {
                return palette;
            }
        }

        // 2. Try external theme file
        if let Some(path) = Self::external_theme_path(name) {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if let Ok(palette) = ColorPalette::from_json(&contents) {
                    return palette;
                }
            }
        }

        // 3. Fallback to default (One Half Dark)
        ColorPalette::from_json(embedded::ONE_HALF_DARK).expect("Default theme should always parse")
    }

    /// Get path to external theme file
    fn external_theme_path(name: &str) -> Option<PathBuf> {
        let config_dir = dirs::home_dir()?
            .join(".config")
            .join("anthropic-spy")
            .join("themes");

        // Try exact name match first
        let exact_path = config_dir.join(format!("{}.json", name));
        if exact_path.exists() {
            return Some(exact_path);
        }

        // Try with spaces replaced by underscores
        let normalized = name.replace(' ', "_");
        let normalized_path = config_dir.join(format!("{}.json", normalized));
        if normalized_path.exists() {
            return Some(normalized_path);
        }

        None
    }

    /// Create theme from palette with configuration
    fn from_palette_with_config(palette: ColorPalette, config: &ThemeConfig) -> Self {
        let semantic = SemanticTheme::from_palette(&palette);

        // Apply background preference
        let background = if config.use_theme_background {
            semantic.background
        } else {
            Color::Reset // Use terminal's default
        };

        Self {
            name: palette.name.clone(),

            // Event colors
            tool_call: semantic.tool_call,
            tool_result_ok: semantic.tool_result_ok,
            tool_result_fail: semantic.tool_result_fail,
            request: semantic.request,
            response: semantic.response,
            error: semantic.error,
            thinking: semantic.thinking,
            api_usage: semantic.api_usage,
            headers: semantic.headers,
            rate_limit: semantic.rate_limit,
            context_compact: semantic.context_compact,

            // UI elements
            context_bar_fill: semantic.context_bar_fill,
            context_bar_warn: semantic.context_bar_warn,
            context_bar_danger: semantic.context_bar_danger,
            status_bar: semantic.status_bar,
            title: semantic.title,
            border: semantic.border,
            highlight: semantic.highlight,

            // Panels
            panel_events: semantic.panel_events,
            panel_thinking: semantic.panel_thinking,
            panel_logs: semantic.panel_logs,
            panel_detail: semantic.panel_detail,

            // Terminal
            background,
            foreground: semantic.foreground,

            // Keep palette for export
            palette,
        }
    }

    /// Export theme as VHS-compatible JSON
    pub fn to_vhs_json(&self) -> String {
        self.palette.to_vhs_json()
    }

    /// Get the underlying color palette
    pub fn palette(&self) -> &ColorPalette {
        &self.palette
    }

    /// Get border color for a panel based on focus state
    pub fn panel_border(&self, panel: crate::tui::scroll::FocusablePanel, focused: bool) -> Color {
        if focused {
            match panel {
                crate::tui::scroll::FocusablePanel::Events => self.panel_events,
                crate::tui::scroll::FocusablePanel::Thinking => self.panel_thinking,
                crate::tui::scroll::FocusablePanel::Logs => self.panel_logs,
                crate::tui::scroll::FocusablePanel::Detail => self.panel_detail,
            }
        } else {
            self.border
        }
    }

    /// List all available themes (embedded + external)
    pub fn list_available() -> Vec<String> {
        let mut themes: Vec<String> = embedded::list_embedded_themes()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Add external themes
        if let Some(config_dir) = dirs::home_dir() {
            let themes_dir = config_dir
                .join(".config")
                .join("anthropic-spy")
                .join("themes");
            if let Ok(entries) = std::fs::read_dir(themes_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.path().file_stem() {
                        if entry
                            .path()
                            .extension()
                            .map(|e| e == "json")
                            .unwrap_or(false)
                        {
                            let name_str = name.to_string_lossy().to_string();
                            // Avoid duplicates with embedded themes
                            if !themes.iter().any(|t| t.eq_ignore_ascii_case(&name_str)) {
                                themes.push(name_str);
                            }
                        }
                    }
                }
            }
        }

        themes
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::by_name("One Half Dark")
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VHS Theme Export Helper
// ─────────────────────────────────────────────────────────────────────────────

/// Write VHS theme JSON file for demo recordings
pub fn export_vhs_theme(theme: &Theme, path: &std::path::Path) -> std::io::Result<()> {
    std::fs::write(path, theme.to_vhs_json())
}
