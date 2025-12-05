// Title bar component
//
// Renders the app title with streaming indicator and conversation topic.

use crate::tui::app::{App, StreamingState};
use crate::tui::layout::Breakpoint;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the title bar at the top of the screen
///
/// Shows:
/// - App name ("Aspy")
/// - Streaming indicator (spinner + state) when active
/// - Conversation topic (extracted from Haiku summarization)
/// - Zoom indicator (right side, hidden on small screens)
pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let bp = Breakpoint::from_width(area.width);

    // Build streaming indicator
    let streaming_indicator = match app.streaming_state() {
        StreamingState::Idle => String::new(),
        StreamingState::Thinking => format!(" {} thinking", app.spinner_char()),
        StreamingState::Generating => format!(" {} generating", app.spinner_char()),
        StreamingState::Executing => format!(" {} executing", app.spinner_char()),
    };

    let title_text = match &app.topic.title {
        Some(topic) => {
            let indicator = if app.topic.is_new_topic { "●" } else { "◦" };
            format!(
                " 🐍 Aspy{} ──── {} {}",
                streaming_indicator, indicator, topic
            )
        }
        None => format!(" 🐍 Aspy{}", streaming_indicator),
    };

    // Build right-side indicator (zoom or help hint)
    // Only show on Normal+ screens to preserve space for title/topic
    let right_indicator = if bp.at_least(Breakpoint::Normal) {
        if let Some(label) = app.zoom_label() {
            format!(" 🔍 {} ", label)
        } else {
            " ? ".to_string()
        }
    } else {
        String::new()
    };

    let title = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(app.theme.title)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(app.theme.border_type)
                .border_style(Style::default().fg(app.theme.title))
                .title_top(ratatui::text::Line::from(right_indicator).right_aligned()),
        );

    f.render_widget(title, area);
}
