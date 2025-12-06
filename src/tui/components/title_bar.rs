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
/// - Session indicator (right side, when multiple sessions active)
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

    // Build session indicator (show when we have sessions)
    let session_indicator = if let Some(session) = app.effective_session() {
        let count = app.session_count();
        if count > 1 {
            // Multiple sessions: show name and count
            format!("[{}] ({}) ", session, count)
        } else {
            // Single session: just show name
            format!("[{}] ", session)
        }
    } else {
        String::new()
    };

    // Build right-side indicator (session + zoom/help)
    // Only show on Normal+ screens to preserve space for title/topic
    let right_indicator = if bp.at_least(Breakpoint::Normal) {
        let zoom_part = if let Some(label) = app.zoom_label() {
            format!("🔍 {} ", label)
        } else {
            "? ".to_string()
        };
        format!(" {}{}", session_indicator, zoom_part)
    } else if !session_indicator.is_empty() {
        // On smaller screens, still show session but skip zoom/help
        format!(" {}", session_indicator)
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
