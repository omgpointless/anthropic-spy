// Markdown parsing for thinking panel
//
// Uses pulldown-cmark to parse markdown and convert to styled ratatui Spans.
// Currently handles: inline code, fenced code blocks, regular text.
// Future: headers, emphasis, lists.

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// A segment of parsed markdown with semantic meaning
#[derive(Debug, Clone)]
pub enum StyledSegment {
    /// Regular text
    Text(String),
    /// Inline code: `like this`
    InlineCode(String),
    /// Fenced code block with optional language
    CodeBlock {
        #[allow(dead_code)] // Future: syntax highlighting
        lang: Option<String>,
        code: String,
    },
    /// Soft break (single newline in source)
    SoftBreak,
    /// Hard break (explicit line break)
    HardBreak,
    /// End of paragraph (adds blank line for spacing)
    ParagraphEnd,
    /// Heading with level
    Heading { level: u8, text: String },
    /// List item marker (bullet or number)
    ListItemStart {
        ordered: bool,
        number: u32,
        depth: usize,
    },
    /// End of list item
    ListItemEnd,
}

/// Parse markdown into styled segments
pub fn parse_markdown(markdown: &str) -> Vec<StyledSegment> {
    let mut segments = Vec::new();
    let mut in_code_block = false;
    let mut in_heading: Option<u8> = None;
    let mut current_lang: Option<String> = None;
    let mut code_block_content = String::new();
    let mut heading_content = String::new();
    // List tracking: stack of (ordered, current_number) for nested lists
    let mut list_stack: Vec<(bool, u32)> = Vec::new();

    for event in Parser::new(markdown) {
        match event {
            // Inline code: `filename.rs`
            Event::Code(code) => {
                if in_heading.is_some() {
                    heading_content.push_str(&code);
                } else {
                    segments.push(StyledSegment::InlineCode(code.to_string()));
                }
            }

            // Heading start
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = Some(match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                });
                heading_content.clear();
            }

            // Heading end
            Event::End(TagEnd::Heading(_)) => {
                if let Some(level) = in_heading.take() {
                    segments.push(StyledSegment::Heading {
                        level,
                        text: heading_content.clone(),
                    });
                }
                heading_content.clear();
            }

            // Fenced code block start
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                current_lang = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        let lang_str = lang.to_string();
                        if lang_str.is_empty() {
                            None
                        } else {
                            Some(lang_str)
                        }
                    }
                    CodeBlockKind::Indented => None,
                };
                code_block_content.clear();
            }

            // Text inside code block - accumulate
            Event::Text(text) if in_code_block => {
                code_block_content.push_str(&text);
            }

            // Text inside heading - accumulate
            Event::Text(text) if in_heading.is_some() => {
                heading_content.push_str(&text);
            }

            // Regular text
            Event::Text(text) => {
                segments.push(StyledSegment::Text(text.to_string()));
            }

            // Code block end - emit accumulated content
            Event::End(TagEnd::CodeBlock) => {
                segments.push(StyledSegment::CodeBlock {
                    lang: current_lang.take(),
                    code: code_block_content.clone(),
                });
                in_code_block = false;
                code_block_content.clear();
            }

            // Paragraph end - add spacing
            Event::End(TagEnd::Paragraph) => {
                segments.push(StyledSegment::ParagraphEnd);
            }

            // Line breaks
            Event::SoftBreak => {
                if in_heading.is_some() {
                    heading_content.push(' ');
                } else {
                    segments.push(StyledSegment::SoftBreak);
                }
            }
            Event::HardBreak => {
                segments.push(StyledSegment::HardBreak);
            }

            // List start - track if ordered and starting number
            Event::Start(Tag::List(first_number)) => {
                let ordered = first_number.is_some();
                let start = first_number.unwrap_or(1) as u32;
                list_stack.push((ordered, start));
            }

            // List end
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                // Add spacing after list ends (if not nested)
                if list_stack.is_empty() {
                    segments.push(StyledSegment::ParagraphEnd);
                }
            }

            // List item start
            Event::Start(Tag::Item) => {
                let depth = list_stack.len();
                if let Some((ordered, ref mut number)) = list_stack.last_mut() {
                    segments.push(StyledSegment::ListItemStart {
                        ordered: *ordered,
                        number: *number,
                        depth,
                    });
                    *number += 1;
                }
            }

            // List item end
            Event::End(TagEnd::Item) => {
                segments.push(StyledSegment::ListItemEnd);
            }

            _ => {}
        }
    }

    segments
}

/// Convert parsed segments to ratatui Lines for rendering
///
/// TODO(human): Implement the segment-to-style mapping
pub fn segments_to_lines(segments: &[StyledSegment], _width: usize) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut current_spans: Vec<Span<'static>> = Vec::new();

    for segment in segments {
        match segment {
            StyledSegment::Text(text) => {
                // Split on newlines to handle multi-line text
                let parts: Vec<&str> = text.split('\n').collect();
                for (i, part) in parts.iter().enumerate() {
                    if !part.is_empty() {
                        current_spans.push(Span::raw(part.to_string()));
                    }
                    // Newline in text = new line (except for last part)
                    if i < parts.len() - 1 {
                        lines.push(Line::from(std::mem::take(&mut current_spans)));
                    }
                }
            }

            StyledSegment::InlineCode(code) => {
                // Inline code - distinct color, no backticks (pulldown-cmark strips them)
                current_spans.push(Span::styled(
                    code.clone(),
                    Style::default().fg(Color::Yellow),
                ));
            }

            StyledSegment::CodeBlock { lang: _, code } => {
                // Flush current line before code block
                if !current_spans.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current_spans)));
                }

                // TODO(human): Style code blocks with theme colors
                // Hint: Different background, maybe dim foreground
                for line in code.lines() {
                    lines.push(Line::from(Span::styled(
                        format!("  {}", line),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
                    )));
                }
            }

            StyledSegment::SoftBreak | StyledSegment::HardBreak => {
                lines.push(Line::from(std::mem::take(&mut current_spans)));
            }

            StyledSegment::ParagraphEnd => {
                // Flush current line and add blank line for paragraph spacing
                if !current_spans.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current_spans)));
                }
                lines.push(Line::from(""));
            }

            StyledSegment::Heading { level, text } => {
                // Flush current line before heading
                if !current_spans.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current_spans)));
                }
                // Style heading based on level
                let style = match level {
                    1 => Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                    2 => Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                    _ => Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                };
                lines.push(Line::from(Span::styled(text.clone(), style)));
            }

            StyledSegment::ListItemStart {
                ordered,
                number,
                depth,
            } => {
                // Flush current spans before list item
                if !current_spans.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current_spans)));
                }
                // Indent based on depth (2 spaces per level, depth starts at 1)
                let indent = "  ".repeat(depth.saturating_sub(1));
                // Add the bullet/number as prefix
                let marker = if *ordered {
                    format!("{}{}. ", indent, number)
                } else {
                    format!("{}• ", indent)
                };
                current_spans.push(Span::styled(marker, Style::default().fg(Color::DarkGray)));
            }

            StyledSegment::ListItemEnd => {
                // Flush the list item line
                if !current_spans.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current_spans)));
                }
            }
        }
    }

    // Don't forget remaining spans
    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }

    lines
}

/// High-level: parse markdown and convert directly to Lines
pub fn render_markdown(markdown: &str, width: usize) -> Vec<Line<'static>> {
    let segments = parse_markdown(markdown);
    segments_to_lines(&segments, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_inline_code() {
        let md = "Check the `main.rs` file";
        let segments = parse_markdown(md);

        assert!(matches!(segments[0], StyledSegment::Text(_)));
        assert!(matches!(segments[1], StyledSegment::InlineCode(_)));
        assert!(matches!(segments[2], StyledSegment::Text(_)));
    }

    #[test]
    fn test_parse_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let segments = parse_markdown(md);

        assert!(matches!(
            &segments[0],
            StyledSegment::CodeBlock { lang: Some(l), .. } if l == "rust"
        ));
    }

    #[test]
    fn test_render_produces_lines() {
        let md = "Hello `world`\n\nNew paragraph";
        let lines = render_markdown(md, 80);

        assert!(!lines.is_empty());
    }
}
