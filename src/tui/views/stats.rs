// Stats view - session profile and analytics
//
// Displays a 2x2 grid of panels:
// - Models: API call distribution by model type
// - Tokens: Token usage breakdown with cache efficiency
// - Tools: Tool call distribution with avg duration
// - Totals: Session summary (requests, cost, thinking)

use crate::events::Stats;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// Import shared formatters from components
use super::super::components::{format_compact_number, format_number};

/// Main render function for the Stats view
pub fn render(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    let stats = &app.stats;

    // Split into 2x2 grid - equal quadrants
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[0]);

    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    // Render each quadrant
    render_models_panel(f, top_row[0], stats);
    render_tokens_panel(f, top_row[1], stats);
    render_tools_panel(f, bottom_row[0], stats);
    render_totals_panel(f, bottom_row[1], stats);
}

/// Render the Models distribution panel with visual bars
fn render_models_panel(f: &mut Frame, area: Rect, stats: &Stats) {
    let mut lines: Vec<Line> = vec![];

    if stats.model_calls.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No API calls yet",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        let total_calls: u32 = stats.model_calls.values().sum();

        // Sort by count descending
        let mut sorted: Vec<_> = stats.model_calls.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        for (model, count) in sorted {
            let pct = (*count as f64 / total_calls as f64) * 100.0;
            let bar = ascii_bar(pct, 20);

            // Color based on model type
            let color = model_color(model);

            // Shorten model name for display
            let short_name = shorten_model_name(model);

            lines.push(Line::from(vec![
                Span::styled(format!("  {:>8} ", short_name), Style::default().fg(color)),
                Span::styled(bar, Style::default().fg(color)),
                Span::styled(
                    format!(" {:>3.0}% ", pct),
                    Style::default().fg(Color::White),
                ),
                Span::styled(
                    format!("({} calls)", count),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(" Models "),
    );

    f.render_widget(paragraph, area);
}

/// Render the Token Breakdown panel with proportional colored bars
fn render_tokens_panel(f: &mut Frame, area: Rect, stats: &Stats) {
    let mut lines: Vec<Line> = vec![];

    let cached = stats.total_cache_read_tokens;
    let input = stats.total_input_tokens;
    let output = stats.total_output_tokens;
    let total = cached + input + output;

    if total == 0 {
        lines.push(Line::from(Span::styled(
            "  No token data yet",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        // Calculate proportions (bar width = 16 chars)
        let bar_width: usize = 16;

        // Helper to render a token bar with consistent style
        let render_bar = |label: &str, value: u64, color: Color| -> Line {
            let pct = (value as f64 / total as f64) * 100.0;
            let filled = ((pct / 100.0) * bar_width as f64).round() as usize;
            let filled = if value > 0 {
                filled.max(1).min(bar_width - 1)
            } else {
                0
            };
            let empty = bar_width - filled;

            Line::from(vec![
                Span::styled(format!("  {:7} ", label), Style::default().fg(color)),
                Span::styled("█".repeat(filled), Style::default().fg(color)),
                Span::styled("░".repeat(empty), Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!(" {:>7} ", format_compact_number(value)),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:>5.1}%", pct),
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        };

        lines.push(render_bar("Cached", cached, Color::Green));
        lines.push(render_bar("Input", input, Color::Cyan));
        lines.push(render_bar("Output", output, Color::Magenta));

        // Blank line
        lines.push(Line::from(""));

        // Cache efficiency and savings row
        let cache_rate = stats.cache_hit_rate();
        let cache_color = if cache_rate >= 90.0 {
            Color::Green
        } else if cache_rate >= 70.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        lines.push(Line::from(vec![
            Span::styled("  Cache: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.1}%", cache_rate),
                Style::default()
                    .fg(cache_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("   Saved: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("${:.2}", stats.cache_savings()),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(" Tokens "),
    );

    f.render_widget(paragraph, area);
}

/// Render the Tools distribution panel
fn render_tools_panel(f: &mut Frame, area: Rect, stats: &Stats) {
    let mut lines: Vec<Line> = vec![];

    if stats.tool_calls_by_name.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No tool calls yet",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        // Sort by count descending
        let mut sorted: Vec<_> = stats.tool_calls_by_name.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        for (tool, count) in sorted {
            let avg_ms = stats.avg_tool_duration_ms(tool).unwrap_or(0);
            let color = tool_color(tool);

            let duration_str = if avg_ms > 1000 {
                format!("{:.1}s", avg_ms as f64 / 1000.0)
            } else {
                format!("{}ms", avg_ms)
            };

            lines.push(Line::from(vec![
                Span::styled(format!("  {:>10} ", tool), Style::default().fg(color)),
                Span::styled(
                    format!("{:>3}", count),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" calls  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("~{}", duration_str),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Tools "),
    );

    f.render_widget(paragraph, area);
}

/// Render the Totals panel
fn render_totals_panel(f: &mut Frame, area: Rect, stats: &Stats) {
    let lines = vec![
        Line::from(vec![
            Span::styled("  Requests:   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", stats.total_requests),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Tool calls: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", stats.total_tool_calls),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Tokens:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format_number(stats.total_tokens()),
                Style::default().fg(Color::LightBlue),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Est. Cost:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("${:.4}", stats.total_cost()),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Thinking:   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} blocks", stats.thinking_blocks),
                Style::default().fg(Color::Magenta),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(" Session Totals "),
    );

    f.render_widget(paragraph, area);
}

// ============================================================================
// Helper functions
// ============================================================================

/// Generate ASCII progress bar
fn ascii_bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f64) as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

/// Shorten model name for display
fn shorten_model_name(model: &str) -> &str {
    if model.contains("haiku") {
        "Haiku"
    } else if model.contains("opus") {
        "Opus"
    } else if model.contains("sonnet") {
        "Sonnet"
    } else {
        model.split('-').next().unwrap_or(model)
    }
}

/// Get color for model type
fn model_color(model: &str) -> Color {
    if model.contains("haiku") {
        Color::Cyan
    } else if model.contains("opus") {
        Color::Magenta
    } else {
        Color::Yellow // Sonnet or other
    }
}

/// Get color for tool type
fn tool_color(tool: &str) -> Color {
    match tool {
        "Read" => Color::Blue,
        "Edit" | "Write" => Color::Green,
        "Bash" => Color::Yellow,
        "Glob" | "Grep" => Color::Cyan,
        "TodoWrite" => Color::Magenta,
        _ => Color::White,
    }
}
