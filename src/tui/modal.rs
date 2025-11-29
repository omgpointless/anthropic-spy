// Modal system for TUI overlays
//
// Self-contained modal dialogs that handle their own input and return actions.
// App just holds Option<Modal>, input routing acts on returned ModalAction.

use crate::theme::list_embedded_themes;
use crossterm::event::KeyCode;

/// Available themes - re-exported from theme module for UI access
pub fn theme_list() -> &'static [&'static str] {
    list_embedded_themes()
}

/// Actions returned by modal input handling
#[derive(Debug, Clone)]
pub enum ModalAction {
    /// Input consumed, no state change needed
    None,
    /// Modal closed, restore original theme (theme name)
    Cancel(String),
    /// Theme was selected and confirmed (already applied via Preview)
    Apply,
    /// Preview theme changed - caller should create Theme with their config
    Preview(String), // theme name
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
        // Case-insensitive match since theme names may vary in casing
        let selected = theme_list()
            .iter()
            .position(|&t| t.eq_ignore_ascii_case(current_theme))
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
                // Return preview theme name for live update
                ModalAction::Preview(self.selected_theme_name().to_string())
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
                // Return preview theme name for live update
                ModalAction::Preview(self.selected_theme_name().to_string())
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
        if self.selected < theme_list().len() - 1 {
            self.selected += 1;
        }
    }

    /// Get the currently selected theme name
    pub fn selected_theme_name(&self) -> &str {
        theme_list()[self.selected]
    }

    /// Get the number of available themes
    pub fn theme_count(&self) -> usize {
        theme_list().len()
    }
}
