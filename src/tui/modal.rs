// Modal system for TUI overlays
//
// Self-contained modal dialogs that handle their own input and return actions.
// App just holds Option<Modal>, input routing acts on returned ModalAction.

use crate::theme::Theme;
use crossterm::event::KeyCode;

/// Available themes (const for zero-allocation lookups)
pub const THEME_LIST: &[&str] = &[
    "basic",
    "terminal",
    "dracula",
    "monokai",
    "monokai-pro-gogh",
    "nord",
    "gruvbox",
];

/// Actions returned by modal input handling
#[derive(Debug, Clone)]
pub enum ModalAction {
    /// Input consumed, no state change needed
    None,
    /// Modal closed, restore original theme
    Cancel(String), // original theme name to restore
    /// Theme was selected and confirmed (already applied via Preview)
    Apply,
    /// Preview theme changed (live preview)
    Preview(Theme),
}

/// Available modal types
#[derive(Debug, Clone)]
pub enum Modal {
    /// Theme selector with list of available themes
    ThemeSelector(ThemeSelectorState),
}

impl Modal {
    /// Create a new theme selector modal, pre-selecting current theme
    pub fn theme_selector(current_theme: &str) -> Self {
        Modal::ThemeSelector(ThemeSelectorState::new(current_theme))
    }

    /// Handle keyboard input, return action for caller to execute
    pub fn handle_input(&mut self, key: KeyCode) -> ModalAction {
        match self {
            Modal::ThemeSelector(state) => state.handle_input(key),
        }
    }
}

/// State for the theme selector modal
#[derive(Debug, Clone)]
pub struct ThemeSelectorState {
    /// Currently highlighted theme index
    pub selected: usize,
    /// Original theme name to restore on cancel
    original_theme: String,
}

impl ThemeSelectorState {
    pub fn new(current_theme: &str) -> Self {
        // Find current theme in list, default to 0
        let selected = THEME_LIST
            .iter()
            .position(|&t| t == current_theme)
            .unwrap_or(0);

        Self {
            selected,
            original_theme: current_theme.to_string(),
        }
    }

    /// Handle input, return action
    pub fn handle_input(&mut self, key: KeyCode) -> ModalAction {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_prev();
                // Return preview for live update
                ModalAction::Preview(Theme::by_name(self.selected_theme_name()))
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
                // Return preview for live update
                ModalAction::Preview(Theme::by_name(self.selected_theme_name()))
            }
            KeyCode::Enter => {
                // Confirm selection (theme already applied via preview)
                ModalAction::Apply
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                // Cancel - restore original theme
                ModalAction::Cancel(self.original_theme.clone())
            }
            _ => ModalAction::None,
        }
    }

    /// Move selection up
    fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down
    fn select_next(&mut self) {
        if self.selected < THEME_LIST.len() - 1 {
            self.selected += 1;
        }
    }

    /// Get the currently selected theme name
    pub fn selected_theme_name(&self) -> &str {
        THEME_LIST[self.selected]
    }
}
