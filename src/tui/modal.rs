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
    /// Close the modal
    Close,
}

/// Available modal types
#[derive(Debug, Clone)]
pub enum Modal {
    /// Help overlay - shows keyboard shortcuts
    Help,
}

impl Modal {
    /// Create a help modal
    pub fn help() -> Self {
        Modal::Help
    }

    /// Handle keyboard input, return action for caller to execute
    pub fn handle_input(&mut self, key: KeyCode) -> ModalAction {
        match self {
            Modal::Help => match key {
                KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => ModalAction::Close,
                _ => ModalAction::None,
            },
        }
    }
}
