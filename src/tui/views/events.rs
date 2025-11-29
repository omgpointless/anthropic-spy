// Events view - main proxy event display
//
// Shows:
// - Event list (scrollable, with selection)
// - Detail panel (optional, toggled with Enter)
// - Thinking panel (rendered based on preset layout)
//
// This is the primary view of anthropic-spy, showing all intercepted
// API traffic in real-time.

use crate::events::ProxyEvent;
use crate::theme::Theme;
use crate::tui::app::{App, StreamingState};
use crate::tui::layout::Breakpoint;
use crate::tui::markdown;
use crate::tui::preset::{LayoutDirection, Panel};
use crate::tui::scroll::FocusablePanel;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Wrap,
    },
    Frame,
};

// Import shared formatters from ui.rs
use super::super::ui::format_number;

/// Main render function for the Events view
pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let bp = Breakpoint::from_width(area.width);

    // Get layout from preset
    let resolved = app.preset.events_view.layout.resolve(bp);
    let direction = match app.preset.events_view.layout.direction {
        LayoutDirection::Horizontal => Direction::Horizontal,
        LayoutDirection::Vertical => Direction::Vertical,
    };

    // Build constraints from resolved layout
    let constraints: Vec<Constraint> = resolved.iter().map(|(_, c)| *c).collect();

    // Split area based on preset layout
    let chunks = Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area);

    // Render each panel by its position in the preset
    for (i, (panel, _)) in resolved.iter().enumerate() {
        match panel {
            Panel::Events => {
                if app.show_detail {
                    render_split_view(f, chunks[i], app);
                } else {
                    render_list_view(f, chunks[i], app);
                }
            }
            Panel::Thinking => render_thinking_panel(f, chunks[i], app),
            _ => {} // Other panels not used in events_view
        }
    }
}

// ============================================================================
// Event list rendering
// ============================================================================

/// Render the main list view showing all events
fn render_list_view(f: &mut Frame, area: Rect, app: &App) {
    let height = area.height.saturating_sub(2) as usize;
    let (start, end) = app.visible_range(height);

    let items: Vec<ListItem> = app.events[start..end]
        .iter()
        .enumerate()
        .map(|(idx, event)| {
            let actual_idx = start + idx;
            let is_selected = actual_idx == app.selected;

            let line = format_event_line(event);
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                event_color_style(event, &app.theme)
            };

            ListItem::new(line).style(style)
        })
        .collect();

    let title = if app.events.is_empty() {
        " Events ".to_string()
    } else if app.events.len() > height && app.selected < app.events.len().saturating_sub(1) {
        format!(" Events ({}/{}) ", app.selected + 1, app.events.len())
    } else {
        format!(" Events ({}) ", app.events.len())
    };

    let focused = app.is_focused(FocusablePanel::Events);
    let border_color = app.theme.panel_border(FocusablePanel::Events, focused);
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(title),
    );

    f.render_widget(list, area);

    // Render scrollbar if content overflows
    let total_events = app.events.len();
    if total_events > height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state =
            ScrollbarState::new(total_events.saturating_sub(height)).position(start);

        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

/// Render split view with list on top and details on bottom
fn render_split_view(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_list_view(f, chunks[0], app);

    if let Some(event) = app.selected_event() {
        render_detail_view(f, chunks[1], app, event);
    }
}

/// Render detailed view of a single event
fn render_detail_view(f: &mut Frame, area: Rect, app: &App, event: &ProxyEvent) {
    let detail_text = format_event_detail(event);

    let lines: Vec<&str> = detail_text.lines().collect();
    let total_lines = lines.len();

    let height = area.height.saturating_sub(2) as usize;
    let start = app
        .panels
        .detail
        .offset()
        .min(total_lines.saturating_sub(height));
    let end = (start + height).min(total_lines);

    let visible_text = lines[start..end].join("\n");

    let focused = app.is_focused(FocusablePanel::Detail);
    let border_color = app.theme.panel_border(FocusablePanel::Detail, focused);
    let paragraph = Paragraph::new(visible_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(" Event Details - Press Esc to close "),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);

    if total_lines > height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state =
            ScrollbarState::new(total_lines.saturating_sub(height)).position(start);

        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

// ============================================================================
// Thinking panel
// ============================================================================

/// Render the thinking panel showing Claude's reasoning
fn render_thinking_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let thinking_content = app.current_thinking_content();
    let is_thinking = app.streaming_state() == StreamingState::Thinking;

    let height = area.height.saturating_sub(2) as usize;
    let width = area.width.saturating_sub(2) as usize;

    // Parse markdown and convert to styled lines
    let styled_lines = if let Some(ref content) = thinking_content {
        markdown::render_markdown(content, width)
    } else {
        Vec::new()
    };
    let line_count = styled_lines.len();
    let text_lines = thinking_content
        .as_ref()
        .map(|c| c.lines().count())
        .unwrap_or(0);

    // Update scroll state dimensions
    app.panels.thinking.update_dimensions(line_count, height);

    let scroll_offset = app.panels.thinking.offset();

    let scroll_indicator = if !app.panels.thinking.auto_follow {
        " [scroll]"
    } else {
        ""
    };

    let title = if is_thinking {
        format!(" 💭 Thinking{} ", app.thinking_dots())
    } else if text_lines > height {
        format!(
            " 💭 Thinking ({} lines, ~{} tok){} ",
            text_lines, app.stats.thinking_tokens, scroll_indicator
        )
    } else if app.stats.thinking_tokens > 0 {
        format!(
            " 💭 Thinking (~{} tok){} ",
            app.stats.thinking_tokens, scroll_indicator
        )
    } else {
        " 💭 Thinking ".to_string()
    };

    let focused = app.is_focused(FocusablePanel::Thinking);
    let border_color = app.theme.panel_border(FocusablePanel::Thinking, focused);

    let paragraph = Paragraph::new(styled_lines)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(title),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset as u16, 0));

    f.render_widget(paragraph, area);

    if line_count > height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state =
            ScrollbarState::new(line_count.saturating_sub(height)).position(scroll_offset);

        f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

// ============================================================================
// Event formatting
// ============================================================================

/// Format an event as a single line for the list view
fn format_event_line(event: &ProxyEvent) -> String {
    match event {
        ProxyEvent::ToolCall {
            timestamp,
            tool_name,
            id,
            ..
        } => {
            format!(
                "[{}] 🔧 Tool Call: {} ({})",
                timestamp.format("%H:%M:%S"),
                tool_name,
                &id[..8]
            )
        }
        ProxyEvent::ToolResult {
            timestamp,
            tool_name,
            duration,
            success,
            ..
        } => {
            let status = if *success { "✓" } else { "✗" };
            format!(
                "[{}] {} Tool Result: {} ({:.2}s)",
                timestamp.format("%H:%M:%S"),
                status,
                tool_name,
                duration.as_secs_f64()
            )
        }
        ProxyEvent::Request {
            timestamp,
            method,
            path,
            ..
        } => {
            format!(
                "[{}] ← Request: {} {}",
                timestamp.format("%H:%M:%S"),
                method,
                path
            )
        }
        ProxyEvent::Response {
            timestamp,
            status,
            duration,
            ..
        } => {
            format!(
                "[{}] → Response: {} ({:.2}s)",
                timestamp.format("%H:%M:%S"),
                status,
                duration.as_secs_f64()
            )
        }
        ProxyEvent::Error {
            timestamp, message, ..
        } => {
            format!("[{}] ❌ Error: {}", timestamp.format("%H:%M:%S"), message)
        }
        ProxyEvent::HeadersCaptured {
            timestamp, headers, ..
        } => {
            let beta_info = if !headers.anthropic_beta.is_empty() {
                format!(" [β:{}]", headers.anthropic_beta.join(","))
            } else {
                String::new()
            };
            format!(
                "[{}] 📋 Headers Captured{}",
                timestamp.format("%H:%M:%S"),
                beta_info
            )
        }
        ProxyEvent::RateLimitUpdate {
            timestamp,
            requests_remaining,
            tokens_remaining,
            ..
        } => {
            format!(
                "[{}] ⚖️  Rate Limits: Req={:?} Tok={:?}",
                timestamp.format("%H:%M:%S"),
                requests_remaining,
                tokens_remaining
            )
        }
        ProxyEvent::ApiUsage {
            timestamp,
            input_tokens,
            output_tokens,
            cache_read_tokens,
            ..
        } => {
            if *cache_read_tokens > 0 {
                format!(
                    "[{}] 📊 Usage: {}in + {}out + {}cached",
                    timestamp.format("%H:%M:%S"),
                    format_number(*input_tokens as u64),
                    format_number(*output_tokens as u64),
                    format_number(*cache_read_tokens as u64)
                )
            } else {
                format!(
                    "[{}] 📊 Usage: {}in + {}out",
                    timestamp.format("%H:%M:%S"),
                    format_number(*input_tokens as u64),
                    format_number(*output_tokens as u64)
                )
            }
        }
        ProxyEvent::Thinking {
            timestamp,
            content,
            token_estimate,
        } => {
            let preview: String = content
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(50)
                .collect();
            format!(
                "[{}] 💭 Thinking: {}... (~{} tok)",
                timestamp.format("%H:%M:%S"),
                preview,
                token_estimate
            )
        }
        ProxyEvent::ContextCompact {
            timestamp,
            previous_context,
            new_context,
        } => {
            format!(
                "[{}] 📦 Context Compact: {}K → {}K",
                timestamp.format("%H:%M:%S"),
                previous_context / 1000,
                new_context / 1000
            )
        }
        ProxyEvent::ThinkingStarted { timestamp } => {
            format!("[{}] 💭 Thinking...", timestamp.format("%H:%M:%S"))
        }
    }
}

/// Format an event as detailed text for the detail view
fn format_event_detail(event: &ProxyEvent) -> String {
    match event {
        ProxyEvent::ToolCall {
            id,
            timestamp,
            tool_name,
            input,
        } => {
            format!(
                "Tool Call\n\nID: {}\nTimestamp: {}\nTool: {}\n\nInput:\n{}",
                id,
                timestamp.to_rfc3339(),
                tool_name,
                serde_json::to_string_pretty(input).unwrap_or_else(|_| "N/A".to_string())
            )
        }
        ProxyEvent::ToolResult {
            id,
            timestamp,
            tool_name,
            output,
            duration,
            success,
        } => {
            format!(
                "Tool Result\n\nID: {}\nTimestamp: {}\nTool: {}\nSuccess: {}\nDuration: {:.2}s\n\nOutput:\n{}",
                id,
                timestamp.to_rfc3339(),
                tool_name,
                success,
                duration.as_secs_f64(),
                serde_json::to_string_pretty(output).unwrap_or_else(|_| "N/A".to_string())
            )
        }
        ProxyEvent::Request {
            id,
            timestamp,
            method,
            path,
            body_size,
            body,
        } => {
            let body_content = if let Some(json_body) = body {
                format!(
                    "\n\nRequest Body:\n{}",
                    serde_json::to_string_pretty(json_body)
                        .unwrap_or_else(|_| "Failed to format".to_string())
                )
            } else {
                String::new()
            };

            format!(
                "HTTP Request\n\nID: {}\nTimestamp: {}\nMethod: {}\nPath: {}\nBody Size: {} bytes{}",
                id,
                timestamp.to_rfc3339(),
                method,
                path,
                body_size,
                body_content
            )
        }
        ProxyEvent::Response {
            request_id,
            timestamp,
            status,
            body_size,
            ttfb,
            duration,
            body,
        } => {
            let body_content = if let Some(json_body) = body {
                format!(
                    "\n\nResponse Body:\n{}",
                    serde_json::to_string_pretty(json_body)
                        .unwrap_or_else(|_| "Failed to format".to_string())
                )
            } else {
                String::new()
            };

            format!(
                "HTTP Response\n\nRequest ID: {}\nTimestamp: {}\nStatus: {}\nBody Size: {} bytes\nTTFB: {}ms\nTotal Duration: {:.2}s{}",
                request_id,
                timestamp.to_rfc3339(),
                status,
                body_size,
                ttfb.as_millis(),
                duration.as_secs_f64(),
                body_content
            )
        }
        ProxyEvent::Error {
            timestamp,
            message,
            context,
        } => {
            format!(
                "Error\n\nTimestamp: {}\nMessage: {}\nContext: {}",
                timestamp.to_rfc3339(),
                message,
                context.as_deref().unwrap_or("N/A")
            )
        }
        ProxyEvent::HeadersCaptured {
            timestamp, headers, ..
        } => {
            let beta_features = if !headers.anthropic_beta.is_empty() {
                headers.anthropic_beta.join(", ")
            } else {
                "None".to_string()
            };

            format!(
                "Headers Captured\n\nTimestamp: {}\n\nRequest Headers:\nAPI Version: {}\nBeta Features: {}\nAPI Key Hash: {}\n\nResponse Headers:\nRequest ID: {}\nOrg ID: {}\n\nRate Limits:\nRequests: {}/{} ({}%)\nTokens: {}/{} ({}%)\nReset: {}",
                timestamp.to_rfc3339(),
                headers.anthropic_version.as_deref().unwrap_or("N/A"),
                beta_features,
                headers.api_key_hash.as_deref().unwrap_or("N/A"),
                headers.request_id.as_deref().unwrap_or("N/A"),
                headers.organization_id.as_deref().unwrap_or("N/A"),
                headers.requests_remaining.map(|r| r.to_string()).unwrap_or("?".to_string()),
                headers.requests_limit.map(|l| l.to_string()).unwrap_or("?".to_string()),
                headers.requests_usage_pct().map(|p| format!("{:.1}", p * 100.0)).unwrap_or("?".to_string()),
                headers.tokens_remaining.map(|r| r.to_string()).unwrap_or("?".to_string()),
                headers.tokens_limit.map(|l| l.to_string()).unwrap_or("?".to_string()),
                headers.tokens_usage_pct().map(|p| format!("{:.1}", p * 100.0)).unwrap_or("?".to_string()),
                headers.requests_reset.as_deref().or(headers.tokens_reset.as_deref()).unwrap_or("N/A")
            )
        }
        ProxyEvent::RateLimitUpdate {
            timestamp,
            requests_remaining,
            requests_limit,
            tokens_remaining,
            tokens_limit,
            reset_time,
        } => {
            format!(
                "Rate Limit Update\n\nTimestamp: {}\n\nRequests: {}/{}\nTokens: {}/{}\nReset: {}",
                timestamp.to_rfc3339(),
                requests_remaining
                    .map(|r| r.to_string())
                    .unwrap_or("?".to_string()),
                requests_limit
                    .map(|l| l.to_string())
                    .unwrap_or("?".to_string()),
                tokens_remaining
                    .map(|r| r.to_string())
                    .unwrap_or("?".to_string()),
                tokens_limit
                    .map(|l| l.to_string())
                    .unwrap_or("?".to_string()),
                reset_time.as_deref().unwrap_or("N/A")
            )
        }
        ProxyEvent::ApiUsage {
            timestamp,
            model,
            input_tokens,
            output_tokens,
            cache_creation_tokens,
            cache_read_tokens,
        } => {
            let total =
                *input_tokens + *output_tokens + *cache_creation_tokens + *cache_read_tokens;
            let cost = crate::pricing::calculate_cost(
                model,
                *input_tokens,
                *output_tokens,
                *cache_creation_tokens,
                *cache_read_tokens,
            );
            let cache_savings = if *cache_read_tokens > 0 {
                crate::pricing::calculate_cache_savings(model, *cache_read_tokens)
            } else {
                0.0
            };

            let cache_info = if *cache_read_tokens > 0 || *cache_creation_tokens > 0 {
                format!(
                    "\n\nCache Statistics:\nCache Creation: {} tokens\nCache Read: {} tokens\nCache Savings: ${:.4} (vs regular input)",
                    format_number(*cache_creation_tokens as u64),
                    format_number(*cache_read_tokens as u64),
                    cache_savings
                )
            } else {
                String::new()
            };

            format!(
                "API Usage\n\nTimestamp: {}\nModel: {}\n\nToken Breakdown:\nInput: {} tokens\nOutput: {} tokens\nTotal: {} tokens\n\nEstimated Cost: ${:.4}{}",
                timestamp.to_rfc3339(),
                model,
                format_number(*input_tokens as u64),
                format_number(*output_tokens as u64),
                format_number(total as u64),
                cost,
                cache_info
            )
        }
        ProxyEvent::Thinking {
            timestamp,
            content,
            token_estimate,
        } => {
            format!(
                "💭 Claude's Thinking\n\nTimestamp: {}\nEstimated Tokens: ~{}\n\n─────────────────────────────────────\n\n{}",
                timestamp.to_rfc3339(),
                token_estimate,
                content
            )
        }
        ProxyEvent::ContextCompact {
            timestamp,
            previous_context,
            new_context,
        } => {
            let reduction = previous_context.saturating_sub(*new_context);
            let reduction_pct = if *previous_context > 0 {
                (reduction as f64 / *previous_context as f64) * 100.0
            } else {
                0.0
            };
            format!(
                "📦 Context Compaction Detected\n\n\
                Timestamp: {}\n\n\
                Previous Context: {} tokens ({:.1}K)\n\
                New Context: {} tokens ({:.1}K)\n\
                Reduction: {} tokens ({:.1}%)\n\n\
                Claude Code triggered a context window compaction to\n\
                reduce memory usage and stay within limits.",
                timestamp.to_rfc3339(),
                previous_context,
                *previous_context as f64 / 1000.0,
                new_context,
                *new_context as f64 / 1000.0,
                reduction,
                reduction_pct
            )
        }
        ProxyEvent::ThinkingStarted { timestamp } => {
            format!(
                "💭 Thinking Started\n\nTimestamp: {}\n\nClaude is processing your request...",
                timestamp.to_rfc3339()
            )
        }
    }
}

/// Get appropriate color style for an event
fn event_color_style(event: &ProxyEvent, theme: &Theme) -> Style {
    match event {
        ProxyEvent::ToolCall { .. } => Style::default().fg(theme.tool_call),
        ProxyEvent::ToolResult { success, .. } => {
            if *success {
                Style::default().fg(theme.tool_result_ok)
            } else {
                Style::default().fg(theme.tool_result_fail)
            }
        }
        ProxyEvent::Request { .. } => Style::default().fg(theme.request),
        ProxyEvent::Response { .. } => Style::default().fg(theme.response),
        ProxyEvent::Error { .. } => Style::default()
            .fg(theme.error)
            .add_modifier(Modifier::BOLD),
        ProxyEvent::HeadersCaptured { .. } => Style::default().fg(theme.headers),
        ProxyEvent::RateLimitUpdate { .. } => Style::default().fg(theme.rate_limit),
        ProxyEvent::ApiUsage { .. } => Style::default().fg(theme.api_usage),
        ProxyEvent::Thinking { .. } => Style::default()
            .fg(theme.thinking)
            .add_modifier(Modifier::ITALIC),
        ProxyEvent::ContextCompact { .. } => Style::default()
            .fg(theme.context_compact)
            .add_modifier(Modifier::BOLD),
        ProxyEvent::ThinkingStarted { .. } => Style::default()
            .fg(theme.thinking)
            .add_modifier(Modifier::ITALIC),
    }
}
