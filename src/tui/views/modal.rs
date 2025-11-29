// Modal overlay rendering
//
// Modals are rendered on top of the main content:
// - Help modal: keyboard shortcuts and current config
//
// TODO: Extract from ui.rs in Phase 1

use crate::tui::app::App;
use crate::tui::modal::Modal;
use ratatui::Frame;

/// Render a modal dialog as a centered overlay
/// Currently delegates to ui.rs (temporary bridge)
pub fn render(f: &mut Frame, modal: &Modal, app: &App) {
    super::super::ui::render_modal(f, modal, app);
}
