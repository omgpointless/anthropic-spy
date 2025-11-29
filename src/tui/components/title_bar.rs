// Title bar component
//
// Renders the app title with streaming indicator and conversation topic.

use crate::tui::app::{App, StreamingState};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the title bar at the top of the screen
///
/// Shows:
/// - App name ("Anthropic Spy")
/// - Streaming indicator (spinner + state) when active
/// - Conversation topic (extracted from Haiku summarization)
pub fn render(f: &mut Frame, area: Rect, app: &App) {
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
                " 🔍 Anthropic Spy{} ──── {} {}",
                streaming_indicator, indicator, topic
            )
        }
        None => format!(" 🔍 Anthropic Spy{}", streaming_indicator),
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
                .border_style(Style::default().fg(app.theme.border))
                .title_top(ratatui::text::Line::from(" ? ").right_aligned()),
        );

    f.render_widget(title, area);
}
