// Settings view - configuration UI
//
// Two-panel layout:
// - Left: Category navigation (Appearance, Layout)
// - Right: Options for selected category (themes, presets)

use crate::tui::app::{App, SettingsCategory, SettingsFocus};
use crate::tui::modal::theme_list;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Available presets with their descriptions
const PRESET_LIST: &[(&str, &str)] = &[
    ("classic", "Side-by-side events and thinking"),
    ("reasoning", "Thinking-first, larger reasoning panel"),
    ("debug", "Expanded logs for debugging"),
];

/// Main render function for the Settings view
pub fn render(f: &mut Frame, area: Rect, app: &App) {
    // Split into left nav (fixed) and right content (fill)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(30)])
        .split(area);

    render_categories(f, chunks[0], app);
    render_options(f, chunks[1], app);
}

/// Render the left category navigation panel
fn render_categories(f: &mut Frame, area: Rect, app: &App) {
    let categories = [
        (SettingsCategory::Appearance, "Appearance"),
        (SettingsCategory::Layout, "Layout"),
    ];

    let is_focused = app.settings.focus == SettingsFocus::Categories;
    let border_color = if is_focused {
        app.theme.tool_result_ok // Highlight when focused
    } else {
        app.theme.border
    };

    let items: Vec<ListItem> = categories
        .iter()
        .map(|(cat, name)| {
            let is_selected = app.settings.category == *cat;
            let prefix = if is_selected { " ▸ " } else { "   " };
            let style = if is_selected {
                Style::default()
                    .fg(app.theme.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{}{}", prefix, name)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(" Categories "),
    );

    f.render_widget(list, area);
}

/// Render the right options panel based on selected category
fn render_options(f: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.settings.focus == SettingsFocus::Options;
    let border_color = if is_focused {
        app.theme.tool_result_ok
    } else {
        app.theme.border
    };

    match app.settings.category {
        SettingsCategory::Appearance => {
            render_theme_options(f, area, app, is_focused, border_color);
        }
        SettingsCategory::Layout => {
            render_preset_options(f, area, app, is_focused, border_color);
        }
    }
}

/// Render theme selection options
fn render_theme_options(
    f: &mut Frame,
    area: Rect,
    app: &App,
    is_focused: bool,
    border_color: Color,
) {
    let items: Vec<ListItem> = theme_list()
        .iter()
        .enumerate()
        .map(|(i, &theme_name)| {
            let is_current = theme_name == app.theme.name;
            let is_highlighted = is_focused && i == app.settings.option_index;

            let prefix = if is_current { " ● " } else { "   " };

            let style = if is_highlighted {
                Style::default()
                    .bg(app.theme.highlight)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default()
                    .fg(app.theme.tool_result_ok)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(format!("{}{}", prefix, theme_name)).style(style)
        })
        .collect();

    let title = if is_focused {
        " Theme (↑↓ select, Enter apply) "
    } else {
        " Theme "
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(title),
    );

    f.render_widget(list, area);
}

/// Render preset selection options
fn render_preset_options(
    f: &mut Frame,
    area: Rect,
    app: &App,
    is_focused: bool,
    border_color: Color,
) {
    let items: Vec<ListItem> = PRESET_LIST
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let is_current = *name == app.preset.name;
            let is_highlighted = is_focused && i == app.settings.option_index;

            let prefix = if is_current { " ● " } else { "   " };

            let style = if is_highlighted {
                Style::default()
                    .bg(app.theme.highlight)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default()
                    .fg(app.theme.tool_result_ok)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            // Show name and description
            let text = format!("{}{:<12} {}", prefix, name, desc);
            ListItem::new(text).style(style)
        })
        .collect();

    let title = if is_focused {
        " Layout Preset (↑↓ select, Enter apply) "
    } else {
        " Layout Preset "
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(title),
    );

    f.render_widget(list, area);
}
