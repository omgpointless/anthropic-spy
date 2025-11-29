// UI rendering - thin dispatcher layer
//
// This module re-exports shell components and provides modal rendering.
// The actual component implementations live in tui/components/.

use super::app::App;
use super::modal::Modal;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

// ============================================================================
// Modal rendering
// ============================================================================

/// Render a modal dialog as a centered overlay
pub fn render_modal(f: &mut Frame, modal: &Modal, app: &App) {
    match modal {
        Modal::Help => render_help_modal(f, app),
    }
}

/// Calculate centered rect for modal dialog
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

/// Render the help modal overlay
fn render_help_modal(f: &mut Frame, app: &App) {
    let content = format!(
        r#"
  Views
    F1, e       Events (main view)
    F2, s       Statistics
    F3          Settings

  Navigation
    ↑/↓, j/k    Scroll list / detail
    Enter       Open detail / apply
    Esc         Close / go back
    Home/End    Jump to start/end

  Settings Navigation
    Tab/→       Switch pane focus
    ↑/↓         Navigate options
    Enter       Apply selection

  Events View
    Tab         Cycle panel focus
    Shift+Tab   Focus previous panel

  General
    ?           Toggle this help
    q           Quit

  Mouse
    Scroll      Navigate events

  ──────────────────────────────────
  Theme: {}  |  Preset: {}
"#,
        app.theme.name, app.preset.name
    );

    // Calculate modal size
    let width = 44;
    let height = 30;
    let area = centered_rect(width, height, f.area());

    // Clear the area behind the modal
    f.render_widget(Clear, area);

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.highlight))
                .title(" Help ")
                .title_bottom(Line::from(" Press ? or Esc to close ").centered()),
        );

    f.render_widget(paragraph, area);
}
