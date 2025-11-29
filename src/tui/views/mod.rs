// Views module - screen-level rendering logic
//
// Each view is a full-screen experience within the TUI:
// - Events: Main view showing proxy events, thinking panel, detail view
// - Stats: Session analytics with model/token/tool breakdowns
// - Settings: Configuration UI for themes and presets
//
// This module dispatches to the appropriate view based on app state.

mod events;
mod modal;
mod settings;
mod stats;

use super::app::{App, View};
use super::preset::Panel;
use crate::tui::components;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

/// Main UI render function - called on every frame
///
/// Builds the shell layout from the preset, then dispatches to the appropriate view.
pub fn draw(f: &mut Frame, app: &mut App) {
    // Build shell layout from preset
    // Structure: [header panels...] [content slot] [footer panels...]
    let shell = &app.preset.shell;

    // Collect constraints: headers + content + footers
    let mut constraints: Vec<Constraint> = Vec::new();
    let mut panel_map: Vec<Option<Panel>> = Vec::new();

    // Add header slots
    for slot in &shell.header {
        constraints.push(slot.size.to_constraint());
        panel_map.push(Some(slot.panel));
    }

    // Add content slot (fills remaining space)
    constraints.push(Constraint::Min(10));
    panel_map.push(None); // None = content slot

    // Add footer slots
    for slot in &shell.footer {
        constraints.push(slot.size.to_constraint());
        panel_map.push(Some(slot.panel));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.area());

    // Render each chunk based on panel type
    let mut content_area: Option<Rect> = None;
    for (i, panel) in panel_map.iter().enumerate() {
        match panel {
            Some(Panel::Title) => components::render_title(f, chunks[i], app),
            Some(Panel::Logs) => components::render_logs_panel(f, chunks[i], app),
            Some(Panel::ContextBar) => components::render_context_bar(f, chunks[i], app),
            Some(Panel::Status) => components::render_status(f, chunks[i], app),
            None => content_area = Some(chunks[i]), // Content slot
            _ => {}                                 // Other panels not in shell
        }
    }

    // Render view content in the content slot
    if let Some(area) = content_area {
        match app.view {
            View::Events => events::render(f, area, app),
            View::Stats => stats::render(f, area, app),
            View::Settings => settings::render(f, area, app),
        }
    }

    // Render modal overlay (on top of everything)
    if let Some(ref modal_state) = app.modal {
        modal::render(f, modal_state, app);
    }
}
