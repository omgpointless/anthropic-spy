// Components module - reusable UI building blocks
//
// Shell components are rendered in every view:
// - Title bar: App name, streaming indicator, topic
// - Status bar: Uptime, requests, tools, cost
// - Context bar: Context window usage gauge
// - Logs panel: System log entries
//
// Each component is a focused, single-responsibility module.

pub mod context_bar;
pub mod formatters;
pub mod logs_panel;
pub mod status_bar;
pub mod title_bar;

// Re-export render functions for convenient access
// Usage: components::title_bar::render(f, area, app)
//    or: components::render_title(f, area, app)

use crate::tui::app::App;
use ratatui::{layout::Rect, Frame};

/// Render the title bar (convenience wrapper)
pub fn render_title(f: &mut Frame, area: Rect, app: &App) {
    title_bar::render(f, area, app);
}

/// Render the context bar (convenience wrapper)
pub fn render_context_bar(f: &mut Frame, area: Rect, app: &App) {
    context_bar::render(f, area, app);
}

/// Render the status bar (convenience wrapper)
pub fn render_status(f: &mut Frame, area: Rect, app: &App) {
    status_bar::render(f, area, app);
}

/// Render the logs panel (convenience wrapper)
pub fn render_logs_panel(f: &mut Frame, area: Rect, app: &mut App) {
    logs_panel::render(f, area, app);
}

// Re-export formatters for shared use
pub use formatters::{format_compact_number, format_number};
