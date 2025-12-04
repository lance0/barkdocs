use crate::theme::Theme;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// A heading extracted for the outline
#[derive(Clone, Debug)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub line_number: usize,
}

/// A link found in the document
#[derive(Clone, Debug)]
pub struct Link {
    pub url: String,
    pub text: String,
    pub line_number: usize,
}

/// Inline text style
#[derive(Clone, Debug, Default)]
pub struct SpanStyle {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub link_url: Option<String>,
    pub strikethrough: bool,
}

/// A text span with styling
#[derive(Clone, Debug)]
pub struct StyledSpan {
    pub text: String,
    pub style: SpanStyle,
}

/// List item content
#[derive(Clone, Debug)]
pub struct ListItem {
    pub spans: Vec<StyledSpan>,
}

/// A block of content
#[derive(Clone, Debug)]
pub enum Block {
    Heading {
        level: u8,
        spans: Vec<StyledSpan>,
    },
    Paragraph {
        spans: Vec<StyledSpan>,
    },
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    List {
        ordered: bool,
        start: Option<u64>,
        items: Vec<ListItem>,
    },
    BlockQuote {
        spans: Vec<StyledSpan>,
    },
    HorizontalRule,
}

/// Parsed document ready for rendering
#[derive(Clone, Debug)]
pub struct Document {
    pub blocks: Vec<Block>,
    pub headings: Vec<Heading>,
    pub links: Vec<Link>,
}

impl Document {
    /// Parse markdown source into a Document
    pub fn parse(source: &str) -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(source, options);
        let mut blocks = Vec::new();
        let mut headings = Vec::new();
        let mut links = Vec::new();
        let mut current_line = 0;
        let mut current_link_url: Option<String> = None;

        // State for building blocks
        let mut current_spans: Vec<StyledSpan> = Vec::new();
        let mut current_style = SpanStyle::default();
        let mut in_heading: Option<u8> = None;
        let mut in_paragraph = false;
        let mut in_blockquote = false;
        let mut in_code_block = false;
        let mut code_language: Option<String> = None;
        let mut code_content = String::new();
        let mut _in_list = false;
        let mut list_ordered = false;
        let mut list_start: Option<u64> = None;
        let mut list_items: Vec<ListItem> = Vec::new();
        let mut in_list_item = false;

        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Heading { level, .. } => {
                        in_heading = Some(heading_level_to_u8(level));
                        current_spans.clear();
                    }
                    Tag::Paragraph => {
                        if !in_list_item {
                            in_paragraph = true;
                            current_spans.clear();
                        }
                    }
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        code_language = match kind {
                            pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                                let lang = lang.to_string();
                                if lang.is_empty() {
                                    None
                                } else {
                                    Some(lang)
                                }
                            }
                            pulldown_cmark::CodeBlockKind::Indented => None,
                        };
                        code_content.clear();
                    }
                    Tag::BlockQuote(_) => {
                        in_blockquote = true;
                        current_spans.clear();
                    }
                    Tag::List(start) => {
                        _in_list = true;
                        list_ordered = start.is_some();
                        list_start = start;
                        list_items.clear();
                    }
                    Tag::Item => {
                        in_list_item = true;
                        current_spans.clear();
                    }
                    Tag::Emphasis => {
                        current_style.italic = true;
                    }
                    Tag::Strong => {
                        current_style.bold = true;
                    }
                    Tag::Strikethrough => {
                        current_style.strikethrough = true;
                    }
                    Tag::Link { dest_url, .. } => {
                        current_style.link_url = Some(dest_url.to_string());
                        current_link_url = Some(dest_url.to_string());
                    }
                    _ => {}
                },
                Event::End(tag) => match tag {
                    TagEnd::Heading(_) => {
                        if let Some(level) = in_heading.take() {
                            let text = current_spans
                                .iter()
                                .map(|s| s.text.as_str())
                                .collect::<String>();

                            headings.push(Heading {
                                level,
                                text: text.clone(),
                                line_number: current_line,
                            });

                            blocks.push(Block::Heading {
                                level,
                                spans: std::mem::take(&mut current_spans),
                            });
                            current_line += 1;
                        }
                    }
                    TagEnd::Paragraph => {
                        if in_list_item {
                            // Don't close paragraph in list item
                        } else if in_blockquote {
                            // Don't close paragraph in blockquote yet
                        } else if in_paragraph {
                            in_paragraph = false;
                            blocks.push(Block::Paragraph {
                                spans: std::mem::take(&mut current_spans),
                            });
                            current_line += 1;
                        }
                    }
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        let lines = code_content.lines().count().max(1);
                        blocks.push(Block::CodeBlock {
                            language: code_language.take(),
                            code: std::mem::take(&mut code_content),
                        });
                        current_line += lines + 2; // +2 for fences
                    }
                    TagEnd::BlockQuote(_) => {
                        in_blockquote = false;
                        blocks.push(Block::BlockQuote {
                            spans: std::mem::take(&mut current_spans),
                        });
                        current_line += 1;
                    }
                    TagEnd::List(_) => {
                        _in_list = false;
                        blocks.push(Block::List {
                            ordered: list_ordered,
                            start: list_start,
                            items: std::mem::take(&mut list_items),
                        });
                        current_line += list_items.len();
                    }
                    TagEnd::Item => {
                        in_list_item = false;
                        list_items.push(ListItem {
                            spans: std::mem::take(&mut current_spans),
                        });
                    }
                    TagEnd::Emphasis => {
                        current_style.italic = false;
                    }
                    TagEnd::Strong => {
                        current_style.bold = false;
                    }
                    TagEnd::Strikethrough => {
                        current_style.strikethrough = false;
                    }
                    TagEnd::Link => {
                        current_style.link_url = None;
                        current_link_url = None;
                    }
                    _ => {}
                },
                Event::Text(text) => {
                    if in_code_block {
                        code_content.push_str(&text);
                    } else {
                        // Track links
                        if let Some(url) = &current_link_url {
                            links.push(Link {
                                url: url.clone(),
                                text: text.to_string(),
                                line_number: current_line,
                            });
                        }
                        current_spans.push(StyledSpan {
                            text: text.to_string(),
                            style: current_style.clone(),
                        });
                    }
                }
                Event::Code(code) => {
                    let mut style = current_style.clone();
                    style.code = true;
                    current_spans.push(StyledSpan {
                        text: code.to_string(),
                        style,
                    });
                }
                Event::SoftBreak | Event::HardBreak => {
                    current_spans.push(StyledSpan {
                        text: " ".to_string(),
                        style: current_style.clone(),
                    });
                }
                Event::Rule => {
                    blocks.push(Block::HorizontalRule);
                    current_line += 1;
                }
                _ => {}
            }
        }

        Document { blocks, headings, links }
    }

    /// Render document to displayable lines
    pub fn render(&self, theme: &Theme) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        for block in &self.blocks {
            match block {
                Block::Heading { level, spans } => {
                    let color = match level {
                        1 => theme.heading_1,
                        2 => theme.heading_2,
                        3 => theme.heading_3,
                        _ => theme.heading_other,
                    };

                    let prefix = "#".repeat(*level as usize);
                    let mut line_spans = vec![Span::styled(
                        format!("{} ", prefix),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    )];

                    for span in spans {
                        line_spans.push(render_span(span, theme, Some(color)));
                    }

                    lines.push(Line::from(line_spans));
                    lines.push(Line::from("")); // blank line after heading
                }

                Block::Paragraph { spans } => {
                    let mut line_spans = Vec::new();
                    for span in spans {
                        line_spans.push(render_span(span, theme, None));
                    }
                    lines.push(Line::from(line_spans));
                    lines.push(Line::from("")); // blank line after paragraph
                }

                Block::CodeBlock { language, code } => {
                    // Code fence start
                    let lang_display = language.as_deref().unwrap_or("");
                    lines.push(Line::styled(
                        format!("```{}", lang_display),
                        Style::default().fg(theme.text_muted),
                    ));

                    // Code content
                    for code_line in code.lines() {
                        lines.push(Line::styled(
                            format!("  {}", code_line),
                            Style::default()
                                .fg(theme.code_inline)
                                .bg(theme.code_block_bg),
                        ));
                    }

                    // Code fence end
                    lines.push(Line::styled(
                        "```",
                        Style::default().fg(theme.text_muted),
                    ));
                    lines.push(Line::from("")); // blank line after code block
                }

                Block::List {
                    ordered,
                    start,
                    items,
                } => {
                    let start_num = start.unwrap_or(1);
                    for (i, item) in items.iter().enumerate() {
                        let marker = if *ordered {
                            format!("{}. ", start_num + i as u64)
                        } else {
                            "• ".to_string()
                        };

                        let mut line_spans = vec![Span::styled(
                            marker,
                            Style::default().fg(theme.list_marker),
                        )];

                        for span in &item.spans {
                            line_spans.push(render_span(span, theme, None));
                        }

                        lines.push(Line::from(line_spans));
                    }
                    lines.push(Line::from("")); // blank line after list
                }

                Block::BlockQuote { spans } => {
                    let mut line_spans = vec![Span::styled(
                        "│ ",
                        Style::default().fg(theme.blockquote),
                    )];

                    for span in spans {
                        line_spans.push(Span::styled(
                            span.text.clone(),
                            Style::default()
                                .fg(theme.blockquote)
                                .add_modifier(Modifier::ITALIC),
                        ));
                    }

                    lines.push(Line::from(line_spans));
                    lines.push(Line::from("")); // blank line after blockquote
                }

                Block::HorizontalRule => {
                    lines.push(Line::styled(
                        "────────────────────────────────────────",
                        Style::default().fg(theme.horizontal_rule),
                    ));
                    lines.push(Line::from(""));
                }
            }
        }

        lines
    }

    /// Get total line count (estimated)
    pub fn line_count(&self) -> usize {
        self.render(&Theme::default()).len()
    }

    /// Get the first link on a given line (if any)
    pub fn link_at_line(&self, line: usize) -> Option<&Link> {
        self.links.iter().find(|link| link.line_number == line)
    }
}

/// Convert heading level enum to u8
fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Render a styled span to a ratatui Span
fn render_span(span: &StyledSpan, theme: &Theme, base_color: Option<Color>) -> Span<'static> {
    let mut style = Style::default();

    // Set base color
    if let Some(color) = base_color {
        style = style.fg(color);
    } else {
        style = style.fg(theme.text);
    }

    // Apply modifiers
    if span.style.bold {
        style = style.add_modifier(Modifier::BOLD);
        style = style.fg(theme.strong);
    }

    if span.style.italic {
        style = style.add_modifier(Modifier::ITALIC);
    }

    if span.style.strikethrough {
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }

    if span.style.code {
        style = style.fg(theme.code_inline);
    }

    if span.style.link_url.is_some() {
        style = style.fg(theme.link).add_modifier(Modifier::UNDERLINED);
    }

    Span::styled(span.text.clone(), style)
}
