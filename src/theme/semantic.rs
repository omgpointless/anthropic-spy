// Semantic theme layer
//
// Maps a ColorPalette's 16 ANSI colors to meaningful UI concepts.
// This separation allows any VHS-compatible palette to work with our UI
// without needing to define 25+ color values manually.

use super::palette::ColorPalette;
use ratatui::style::Color;

/// Semantic color assignments for the TUI.
/// Generated automatically from a ColorPalette using consistent mapping rules.
#[derive(Debug, Clone)]
pub struct SemanticTheme {
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
    pub context_bar_fill: Color,   // Progress bar: normal
    pub context_bar_warn: Color,   // Progress bar: warning
    pub context_bar_danger: Color, // Progress bar: danger
    pub status_bar: Color,
    pub title: Color,
    pub border: Color,
    pub highlight: Color,

    // ─── Panel Identity Colors (focused border) ──────────────
    pub panel_events: Color,
    pub panel_thinking: Color,
    pub panel_logs: Color,
    pub panel_detail: Color,

    // ─── Terminal Colors ─────────────────────────────────────
    pub background: Color,
    pub foreground: Color,
    #[allow(dead_code)] // Future: selection highlighting
    pub selection: Color,
}

impl SemanticTheme {
    /// Create semantic theme from a color palette using standard mapping.
    ///
    /// Mapping philosophy (dark themes):
    /// - Cyan family → navigation, interactive elements (tool calls, titles)
    /// - Green family → success, completion (results OK, status)
    /// - Red family → errors, failures, danger states
    /// - Yellow family → warnings, highlights, attention
    /// - Blue family → informational (requests, API usage)
    /// - Purple/Magenta family → special states (thinking, responses)
    pub fn from_palette(p: &ColorPalette) -> Self {
        Self {
            // Event types - use bright variants for visibility
            tool_call: p.cyan,
            tool_result_ok: p.green,
            tool_result_fail: p.red,
            request: p.blue,
            response: p.purple,
            error: p.red,
            thinking: p.purple,
            api_usage: p.bright_blue,
            headers: p.bright_black, // Muted/comment color
            rate_limit: p.yellow,
            context_compact: p.yellow,

            // Progress bar fills - darken for white text contrast
            context_bar_fill: Self::darken(p.green, 0.5),
            context_bar_warn: Self::darken(p.yellow, 0.5),
            context_bar_danger: Self::darken(p.red, 0.5),

            // UI elements
            status_bar: p.green,
            title: p.cyan,
            border: p.white,
            highlight: p.yellow,

            // Panel identity (focused state)
            panel_events: p.cyan,
            panel_thinking: p.purple,
            panel_logs: p.green,
            panel_detail: p.yellow,

            // Terminal integration
            background: p.background,
            foreground: p.foreground,
            selection: p.selection,
        }
    }

    /// Darken an RGB color by a factor (0.0 = black, 1.0 = unchanged)
    fn darken(color: Color, factor: f32) -> Color {
        match color {
            Color::Rgb(r, g, b) => Color::Rgb(
                (r as f32 * factor) as u8,
                (g as f32 * factor) as u8,
                (b as f32 * factor) as u8,
            ),
            // For ANSI colors, return a darkened approximation
            Color::Green => Color::Rgb(0x00, 0x64, 0x00),
            Color::Yellow => Color::Rgb(0x80, 0x80, 0x00),
            Color::Red => Color::Rgb(0x8b, 0x00, 0x00),
            other => other,
        }
    }
}
