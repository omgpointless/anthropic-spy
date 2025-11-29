// Logs panel component
//
// Renders system log entries with color-coded severity levels.

use crate::logging::{LogEntry, LogLevel};
use crate::theme::Theme;
use crate::tui::app::App;
use crate::tui::scroll::FocusablePanel;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Render system logs panel
///
/// Shows:
/// - Timestamped log entries
/// - Color-coded by severity (Error, Warn, Info, Debug, Trace)
/// - Scroll indicator when not at bottom
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let height = area.height.saturating_sub(2) as usize; // Account for borders
    let all_entries = app.log_buffer.get_all();
    let total = all_entries.len();

    // Update scroll state dimensions
    app.panels.logs.update_dimensions(total, height);

    // Get visible range based on scroll position
    let (start, end) = app.panels.logs.visible_range();
    let visible_entries: Vec<_> = all_entries
        .into_iter()
        .skip(start)
        .take(end - start)
        .collect();

    // Convert log entries to list items with color coding
    let items: Vec<ListItem> = visible_entries
        .iter()
        .map(|entry| {
            let formatted = format_log_entry(entry);
            let style = log_level_style(&entry.level, &app.theme);
            ListItem::new(formatted).style(style)
        })
        .collect();

    let focused = app.is_focused(FocusablePanel::Logs);
    let border_color = app.theme.panel_border(FocusablePanel::Logs, focused);

    // Show scroll indicator if not at bottom
    let title = if app.panels.logs.auto_follow {
        " System Logs "
    } else {
        " System Logs [scroll] "
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(title),
    );

    f.render_widget(list, area);
}

/// Format a log entry for display
fn format_log_entry(entry: &LogEntry) -> String {
    format!(
        "[{}] {:5} {}",
        entry.timestamp.format("%H:%M:%S"),
        entry.level.as_str(),
        entry.message
    )
}

/// Get color style for log level (uses theme colors for consistency)
fn log_level_style(level: &LogLevel, theme: &Theme) -> Style {
    match level {
        LogLevel::Error => Style::default()
            .fg(theme.error)
            .add_modifier(Modifier::BOLD),
        LogLevel::Warn => Style::default().fg(theme.rate_limit),
        LogLevel::Info => Style::default().fg(theme.api_usage),
        LogLevel::Debug => Style::default().fg(theme.headers),
        LogLevel::Trace => Style::default().fg(theme.headers),
    }
}
