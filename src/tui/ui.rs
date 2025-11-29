// UI rendering logic
//
// This module contains all the rendering code for the TUI. In ratatui,
// you define the UI layout and widgets in a render function that gets
// called on every frame.

use super::app::{App, StreamingState};
use super::layout::Breakpoint;
use super::modal::Modal;
use super::scroll::FocusablePanel;
use crate::logging::{LogEntry, LogLevel};
use crate::theme::Theme;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph},
    Frame,
};

// ============================================================================
// Shell components (rendered in every view)
// These will be extracted to tui/components/ in Phase 2
// ============================================================================

/// Render the title bar
/// Public for components module re-export
pub fn render_title(f: &mut Frame, area: Rect, app: &App) {
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

/// Render the context window usage bar (1-line gauge with centered text)
/// Public for components module re-export
pub fn render_context_bar(f: &mut Frame, area: Rect, app: &App) {
    let stats = &app.stats;

    let (label, pct, color) = if stats.current_context_tokens > 0 {
        let pct = stats.context_usage_percent().unwrap_or(0.0);
        let over_limit = pct >= 100.0;

        let color = if over_limit {
            // Over limit: compact is pending, use warning color
            app.theme.context_bar_warn
        } else if pct >= 90.0 {
            app.theme.context_bar_danger
        } else if pct >= 70.0 {
            app.theme.context_bar_warn
        } else {
            app.theme.context_bar_fill
        };

        let label = if over_limit {
            // Don't show embarrassing >100%, signal compact is pending
            format!(
                "Context: {} / {} (~100%, compact pending)",
                format_number(stats.current_context_tokens),
                format_number(stats.context_limit()),
            )
        } else {
            format!(
                "Context: {} / {} ({:.1}%)",
                format_number(stats.current_context_tokens),
                format_number(stats.context_limit()),
                pct
            )
        };
        (label, pct.min(100.0), color) // Cap display at 100%
    } else {
        (
            "Context: waiting for API call...".to_string(),
            0.0,
            Color::DarkGray,
        )
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(Color::Black))
        .percent(pct as u16)
        .label(Span::styled(
            label,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));

    f.render_widget(gauge, area);
}

/// Render the status bar with statistics
/// Switches to compact icon-based format when terminal width is narrow
/// Public for components module re-export
pub fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let stats = &app.stats;
    let bp = Breakpoint::from_width(area.width);

    let status_text = if !bp.at_least(Breakpoint::Wide) {
        // Compact format with icons for narrow terminals
        let token_info = if stats.total_tokens() > 0 {
            format!(" │ 💰${:.2}", stats.total_cost())
        } else {
            String::new()
        };

        format!(
            " {} │ 📡{} │ 🔧{} │ ✓{:.0}% │ ~{:.0}ms{}",
            app.uptime(),
            stats.total_requests,
            stats.total_tool_calls,
            stats.success_rate(),
            stats.avg_ttfb().as_millis(),
            token_info,
        )
    } else {
        // Full format for wide terminals
        let token_info = if stats.total_tokens() > 0 {
            let cost = stats.total_cost();
            let savings = stats.cache_savings();

            if savings > 0.0 {
                format!(
                    " | {}tok | ${:.2} | Saved: ${:.2}",
                    format_compact_number(stats.total_tokens()),
                    cost,
                    savings
                )
            } else {
                format!(
                    " | {}tok | ${:.2}",
                    format_compact_number(stats.total_tokens()),
                    cost
                )
            }
        } else {
            String::new()
        };

        let tools_info = if stats.failed_tool_calls > 0 {
            format!("🔧{} ✗{}", stats.total_tool_calls, stats.failed_tool_calls)
        } else {
            format!("🔧{}", stats.total_tool_calls)
        };

        format!(
            " {} │ 📡{} │ {} │ ✓{:.1}% │ ~{}ms{}",
            app.uptime(),
            stats.total_requests,
            tools_info,
            stats.success_rate(),
            stats.avg_ttfb().as_millis(),
            token_info,
        )
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(app.theme.status_bar))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}

/// Format a large number with commas for readability
/// Public for shared use across views
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();

    for (count, ch) in s.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, ch);
    }

    result
}

/// Format a number compactly (e.g., 954356 -> "954K")
/// Public for shared use across views
pub fn format_compact_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{}K", n / 1_000)
    } else {
        n.to_string()
    }
}

// Events view rendering has been extracted to tui/views/events.rs
// The following event list/detail/format functions were removed:
// - render_list_view, render_split_view, render_detail_view
// - format_event_line, format_event_detail, event_color_style

// ============================================================================
// Logs panel (shell component)
// ============================================================================

// NOTE: The event rendering code (~560 lines) was here but has been extracted.
// We need to manually remove the remaining dead code after this marker.

/// Render system logs panel at the bottom of the screen
/// Public for components module re-export
pub fn render_logs_panel(f: &mut Frame, area: Rect, app: &mut App) {
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

// ============================================================================
// Modal rendering
// ============================================================================

/// Render a modal dialog as a centered overlay
/// Public for views module bridge (temporary)
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
