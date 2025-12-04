use crate::app::{AppState, FocusedPanel, InputMode, SplitDirection};
use crate::theme::Theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Wrap,
};

/// Main draw function
pub fn draw(frame: &mut Frame, state: &mut AppState) {
    let area = frame.area();

    // Layout: [Optional Outline Panel] [Main Content]
    let main_chunks = if state.show_outline {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(state.outline_width), Constraint::Min(20)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20)])
            .split(area)
    };

    let (outline_area, content_area) = if state.show_outline {
        (main_chunks[0], main_chunks[1])
    } else {
        (Rect::default(), main_chunks[0])
    };

    // Store outline area for mouse handling
    state.outline_area = outline_area;

    if state.show_outline {
        draw_outline(frame, state, outline_area);
    }

    // Content area layout: header, content, status, search bar
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(3),    // Content
            Constraint::Length(1), // Status bar
            Constraint::Length(1), // Search/message bar
        ])
        .split(content_area);

    draw_header(frame, state, content_chunks[0]);
    draw_content(frame, state, content_chunks[1]);
    draw_status_bar(frame, state, content_chunks[2]);
    draw_search_bar(frame, state, content_chunks[3]);

    // Overlays
    if state.show_help {
        draw_help_overlay(frame, &state.theme);
    }

    if state.show_settings {
        draw_settings_overlay(frame, state);
    }

    if state.show_file_picker {
        draw_file_picker(frame, state);
    }

    if state.show_buffer_list {
        draw_buffer_list(frame, state);
    }

    if state.show_history {
        draw_history_overlay(frame, state);
    }

    if state.show_bookmarks {
        draw_bookmarks_overlay(frame, state);
    }

    if state.show_url_input {
        draw_url_input(frame, state);
    }

    if state.show_bookmark_name_input {
        draw_bookmark_name_input(frame, state);
    }
}

/// Draw the header bar
fn draw_header(frame: &mut Frame, state: &AppState, area: Rect) {
    let theme = &state.theme;

    let filename = state
        .file_path
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "No file".to_string());

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " barkdocs ",
            Style::default()
                .fg(theme.header_title)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("| ", Style::default().fg(theme.text_muted)),
        Span::styled(filename, Style::default().fg(theme.header_filename)),
    ]))
    .style(Style::default().bg(theme.header_bg));

    frame.render_widget(header, area);
}

/// Draw the outline panel
fn draw_outline(frame: &mut Frame, state: &AppState, area: Rect) {
    let theme = &state.theme;
    let focused = state.focused_panel == FocusedPanel::Outline;

    let border_color = if focused {
        theme.border_focused
    } else {
        theme.border_unfocused
    };

    let block = Block::default()
        .title(" Outline ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(doc) = &state.document {
        // Calculate available width for text (panel width - borders - marker)
        let available_width = inner.width.saturating_sub(2) as usize; // 2 for marker "> "

        let items: Vec<ListItem> = doc
            .headings
            .iter()
            .enumerate()
            .map(|(i, heading)| {
                let indent = "  ".repeat((heading.level.saturating_sub(1)) as usize);
                let marker = if i == state.outline_selected {
                    "> "
                } else {
                    "  "
                };

                // Calculate remaining width after indent
                let text_width = available_width.saturating_sub(indent.len());

                // Truncate with ellipsis if needed
                let display_text = if heading.text.len() > text_width && text_width > 3 {
                    format!("{}...", &heading.text[..text_width.saturating_sub(3)])
                } else if heading.text.len() > text_width {
                    heading.text[..text_width].to_string()
                } else {
                    heading.text.clone()
                };

                let style = if i == state.outline_selected {
                    Style::default()
                        .fg(theme.outline_selected)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.outline_heading)
                };

                ListItem::new(format!("{}{}{}", marker, indent, display_text)).style(style)
            })
            .collect();

        let list = List::new(items);
        frame.render_widget(list, inner);
    } else {
        let empty = Paragraph::new("(no document)").style(Style::default().fg(theme.empty_state));
        frame.render_widget(empty, inner);
    }
}

/// Draw the main content area
fn draw_content(frame: &mut Frame, state: &mut AppState, area: Rect) {
    // Handle split view
    let pane_areas = if state.split_direction != SplitDirection::None && state.panes.len() > 1 {
        let direction = match state.split_direction {
            SplitDirection::Vertical => Direction::Horizontal,
            SplitDirection::Horizontal => Direction::Vertical,
            SplitDirection::None => Direction::Horizontal,
        };

        Layout::default()
            .direction(direction)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area)
            .to_vec()
    } else {
        vec![area]
    };

    // Store content areas for mouse handling
    state.content_areas = pane_areas.clone();

    // Render each pane
    for (i, pane_area) in pane_areas.iter().enumerate() {
        if i < state.panes.len() {
            draw_pane(frame, state, *pane_area, i);
        }
    }
}

/// Draw a single content pane
fn draw_pane(frame: &mut Frame, state: &AppState, area: Rect, pane_idx: usize) {
    let theme = &state.theme;
    let pane = &state.panes[pane_idx];
    let is_active = pane_idx == state.active_pane;
    let is_split = state.split_direction != SplitDirection::None;

    // Determine if we need a border
    let inner_area = if is_split {
        let border_color = if is_active {
            theme.border_focused
        } else {
            theme.border_unfocused
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(area);
        frame.render_widget(block, area);
        inner
    } else {
        area
    };

    // Calculate visible lines
    let height = inner_area.height as usize;
    let total_lines = state.rendered_lines.len();
    let scroll = pane.scroll.min(total_lines.saturating_sub(1));

    // Calculate line number gutter width if enabled
    let line_num_width = if state.show_line_numbers {
        // Width needed for largest line number + 1 space
        let max_line = scroll + height;
        let digits = if max_line == 0 {
            1
        } else {
            (max_line as f64).log10().floor() as u16 + 1
        };
        digits + 2 // digits + space + separator
    } else {
        0
    };

    // Split area for line numbers and content
    let (line_num_area, content_area) =
        if state.show_line_numbers && line_num_width < inner_area.width {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(line_num_width), Constraint::Min(1)])
                .split(inner_area);
            (Some(chunks[0]), chunks[1])
        } else {
            (None, inner_area)
        };

    // Get lines to display
    let visible_lines: Vec<Line> = state
        .rendered_lines
        .iter()
        .skip(scroll)
        .take(height)
        .cloned()
        .collect();

    // Apply search highlighting if active
    let display_lines = if !pane.search_matches.is_empty() {
        apply_search_highlighting(&visible_lines, pane, scroll, theme)
    } else {
        visible_lines
    };

    // Render line numbers if enabled
    if let Some(ln_area) = line_num_area {
        let line_numbers: Vec<Line> = (0..height)
            .map(|i| {
                let line_num = scroll + i + 1;
                if line_num <= total_lines {
                    Line::from(Span::styled(
                        format!(
                            "{:>width$} ",
                            line_num,
                            width = (line_num_width - 2) as usize
                        ),
                        Style::default().fg(theme.text_muted),
                    ))
                } else {
                    Line::from(Span::styled(
                        format!("{:>width$} ", "~", width = (line_num_width - 2) as usize),
                        Style::default().fg(theme.text_muted),
                    ))
                }
            })
            .collect();

        let ln_paragraph = Paragraph::new(line_numbers);
        frame.render_widget(ln_paragraph, ln_area);
    }

    // Render content
    let content = if state.line_wrap {
        Paragraph::new(display_lines).wrap(Wrap { trim: false })
    } else {
        // Apply horizontal scroll
        let scrolled_lines: Vec<Line> = display_lines
            .into_iter()
            .map(|line| apply_horizontal_scroll(line, pane.horizontal_scroll))
            .collect();
        Paragraph::new(scrolled_lines)
    };

    frame.render_widget(content, content_area);

    // Render scrollbar if needed
    if total_lines > height {
        let scrollbar_area = Rect {
            x: content_area.x + content_area.width.saturating_sub(1),
            y: content_area.y,
            width: 1,
            height: content_area.height,
        };

        let mut scrollbar_state =
            ScrollbarState::new(total_lines.saturating_sub(height)).position(scroll);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

/// Apply search highlighting to lines
fn apply_search_highlighting(
    lines: &[Line<'static>],
    pane: &crate::app::PaneState,
    scroll_offset: usize,
    theme: &Theme,
) -> Vec<Line<'static>> {
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let line_idx = scroll_offset + i;

            // Check if any matches are on this line
            let matches_on_line: Vec<_> = pane
                .search_matches
                .iter()
                .filter(|m| m.line == line_idx)
                .collect();

            if matches_on_line.is_empty() {
                line.clone()
            } else {
                // Rebuild line with highlighting
                let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
                let mut result_spans = Vec::new();
                let mut last_end = 0;

                for m in matches_on_line {
                    // Text before match
                    if m.start > last_end {
                        result_spans.push(Span::styled(
                            text[last_end..m.start].to_string(),
                            Style::default().fg(theme.text),
                        ));
                    }

                    // Highlighted match
                    result_spans.push(Span::styled(
                        text[m.start..m.end].to_string(),
                        Style::default()
                            .fg(theme.highlight_match_fg)
                            .bg(theme.highlight_match_bg)
                            .add_modifier(Modifier::BOLD),
                    ));

                    last_end = m.end;
                }

                // Text after last match
                if last_end < text.len() {
                    result_spans.push(Span::styled(
                        text[last_end..].to_string(),
                        Style::default().fg(theme.text),
                    ));
                }

                Line::from(result_spans)
            }
        })
        .collect()
}

/// Apply horizontal scroll to a line
fn apply_horizontal_scroll(line: Line<'static>, offset: usize) -> Line<'static> {
    if offset == 0 {
        return line;
    }

    let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
    if offset >= text.len() {
        return Line::from("");
    }

    // Simple approach: just slice the text
    Line::from(text[offset..].to_string())
}

/// Draw the status bar
fn draw_status_bar(frame: &mut Frame, state: &AppState, area: Rect) {
    let theme = &state.theme;
    let pane = state.current_pane();

    // Mode indicator
    let mode_text = match state.mode {
        InputMode::Normal => " NORMAL ",
        InputMode::Search => " SEARCH ",
        InputMode::SplitCommand => " SPLIT ",
        InputMode::UrlInput => " URL ",
        InputMode::BookmarkName => " BOOKMARK ",
    };

    let mode_span = Span::styled(
        mode_text,
        Style::default()
            .fg(theme.status_mode_fg)
            .bg(theme.status_mode_bg)
            .add_modifier(Modifier::BOLD),
    );

    // Line count and position
    let total = state.line_count();
    let current = pane.scroll + 1;
    let position = format!(" {}/{} ", current, total);

    // Flags
    let mut flags = String::new();
    if state.is_loading {
        flags.push_str("[LOADING]");
    }
    if state.is_viewing_url() {
        flags.push_str("[URL]");
    }
    if state.line_wrap {
        flags.push_str("[W]");
    }
    if pane.search_is_regex {
        flags.push_str("[.*]");
    }
    if state.show_line_numbers {
        flags.push_str("[#]");
    }
    if state.split_direction != SplitDirection::None {
        flags.push_str(&format!(
            "[{}/{}]",
            state.active_pane + 1,
            state.panes.len()
        ));
    }
    if state.buffers.len() > 1 {
        flags.push_str(&format!(
            "[B{}/{}]",
            state.active_buffer + 1,
            state.buffers.len()
        ));
    }
    if state.auto_reload {
        flags.push_str("[R]");
    }

    // Help hint
    let hint = match state.mode {
        InputMode::Normal => " ?:help O:url H:history m:bookmarks ",
        InputMode::Search => " Enter:search Esc:cancel Ctrl+r:regex ",
        InputMode::SplitCommand => " v:vsplit s:hsplit q:close w:cycle ",
        InputMode::UrlInput => " Enter:load Esc:cancel ",
        InputMode::BookmarkName => " Enter:save Esc:cancel ",
    };

    // Calculate padding
    let used_width = mode_text.len() + position.len() + flags.len() + hint.len();
    let padding = area.width.saturating_sub(used_width as u16) as usize;

    // Build status line
    let status = Line::from(vec![
        mode_span,
        Span::styled(position, Style::default().fg(theme.text)),
        Span::styled(flags, Style::default().fg(theme.text_muted)),
        Span::styled(" ".repeat(padding), Style::default()),
        Span::styled(hint, Style::default().fg(theme.status_help)),
    ]);

    let paragraph = Paragraph::new(status).style(Style::default().bg(theme.status_bg));

    frame.render_widget(paragraph, area);
}

/// Draw the search/message bar
fn draw_search_bar(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let theme = &state.theme;

    if state.mode == InputMode::Search {
        // Search input mode
        let prefix = Span::styled("/", Style::default().fg(theme.header_title));

        // Render textarea
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(area);

        frame.render_widget(Paragraph::new(Line::from(prefix)), chunks[0]);

        let pane = state.current_pane_mut();
        frame.render_widget(&pane.search_textarea, chunks[1]);
    } else if let Some(msg) = &state.status_message {
        // Status message
        let message =
            Paragraph::new(msg.as_str()).style(Style::default().fg(theme.warning_message));
        frame.render_widget(message, area);
    } else {
        // Empty
        frame.render_widget(Paragraph::new(""), area);
    }
}

/// Draw the help overlay
fn draw_help_overlay(frame: &mut Frame, theme: &Theme) {
    let area = frame.area();

    // Calculate centered area
    let width = 60.min(area.width.saturating_sub(4));
    let height = 30.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let popup_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Draw block
    let block = Block::default()
        .title(" Keyboard Shortcuts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Help content
    let help_text = vec![
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  j/k, ↑/↓       Scroll up/down"),
        Line::from("  h/l, ←/→       Scroll left/right"),
        Line::from("  g/G            Top/bottom"),
        Line::from("  Ctrl+u/d       Half page up/down"),
        Line::from("  Enter/f        Follow link"),
        Line::from("  y              Yank (copy) line"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Search",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  /              Start search"),
        Line::from("  n/N            Next/prev match"),
        Line::from("  Ctrl+r         Toggle regex"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "View",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  b              Toggle outline"),
        Line::from("  < >  [ ]       Resize outline panel"),
        Line::from("  w/#            Line wrap / numbers"),
        Line::from("  Ctrl+s/R       Syntax hl / auto-reload"),
        Line::from("  Tab            Switch panel focus"),
        Line::from("  Ctrl+W,v/s/q   Split v/h / close pane"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Files & URLs",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  o/O            Open file / URL"),
        Line::from("  H              View history"),
        Line::from("  m/M            Bookmarks / add bookmark"),
        Line::from("  B              Buffer list"),
        Line::from("  Ctrl+n/p/x     Next/prev/close buffer"),
        Line::from(""),
        Line::from("  S  Settings    q  Quit    ?  Close"),
    ];

    let paragraph = Paragraph::new(help_text).style(Style::default().fg(theme.text));

    frame.render_widget(paragraph, inner);
}

/// Draw the settings overlay
fn draw_settings_overlay(frame: &mut Frame, state: &AppState) {
    let theme = &state.theme;
    let area = frame.area();

    // Calculate centered area
    let width = 40.min(area.width.saturating_sub(4));
    let height = 14.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let popup_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Draw block
    let block = Block::default()
        .title(" Settings ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Settings list
    let settings = [
        ("Theme", theme.name.to_string()),
        (
            "Line Wrap",
            if state.line_wrap { "ON" } else { "OFF" }.to_string(),
        ),
        (
            "Outline Panel",
            if state.show_outline { "ON" } else { "OFF" }.to_string(),
        ),
        (
            "Line Numbers",
            if state.show_line_numbers { "ON" } else { "OFF" }.to_string(),
        ),
        (
            "Syntax Highlight",
            if state.syntax_highlighting {
                "ON"
            } else {
                "OFF"
            }
            .to_string(),
        ),
        (
            "Auto Reload",
            if state.auto_reload { "ON" } else { "OFF" }.to_string(),
        ),
        ("Save Settings", "[Enter]".to_string()),
    ];

    let items: Vec<ListItem> = settings
        .iter()
        .enumerate()
        .map(|(i, (name, value))| {
            let marker = if i == state.settings_selected {
                "> "
            } else {
                "  "
            };

            let style = if i == state.settings_selected {
                Style::default()
                    .fg(theme.outline_selected)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text)
            };

            ListItem::new(format!("{}{}: {}", marker, name, value)).style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Draw the file picker overlay
fn draw_file_picker(frame: &mut Frame, state: &AppState) {
    let theme = &state.theme;
    let area = frame.area();

    // Calculate centered area
    let width = 50.min(area.width.saturating_sub(4));
    let height = 15.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let popup_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Draw block
    let block = Block::default()
        .title(" Open File ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    if state.file_picker_files.is_empty() {
        let empty =
            Paragraph::new("No markdown files found").style(Style::default().fg(theme.empty_state));
        frame.render_widget(empty, inner);
        return;
    }

    // Calculate visible range (scrolling if needed)
    let visible_height = inner.height as usize;
    let selected = state.file_picker_selected;
    let total = state.file_picker_files.len();

    let start = if selected >= visible_height {
        selected - visible_height + 1
    } else {
        0
    };
    let end = (start + visible_height).min(total);

    // File list
    let items: Vec<ListItem> = state
        .file_picker_files
        .iter()
        .enumerate()
        .skip(start)
        .take(end - start)
        .map(|(i, path)| {
            let filename = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());

            let marker = if i == selected { "> " } else { "  " };

            let style = if i == selected {
                Style::default()
                    .fg(theme.outline_selected)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text)
            };

            ListItem::new(format!("{}{}", marker, filename)).style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Draw the buffer list overlay
fn draw_buffer_list(frame: &mut Frame, state: &AppState) {
    let theme = &state.theme;
    let area = frame.area();

    // Calculate centered area
    let width = 50.min(area.width.saturating_sub(4));
    let height = 15.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let popup_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Draw block
    let block = Block::default()
        .title(format!(" Open Buffers ({}) ", state.buffers.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    if state.buffers.is_empty() {
        let empty = Paragraph::new("No buffers open").style(Style::default().fg(theme.empty_state));
        frame.render_widget(empty, inner);
        return;
    }

    // Calculate visible range (scrolling if needed)
    let visible_height = inner.height as usize;
    let selected = state.buffer_list_selected;
    let total = state.buffers.len();

    let start = if selected >= visible_height {
        selected - visible_height + 1
    } else {
        0
    };
    let end = (start + visible_height).min(total);

    // Buffer list
    let items: Vec<ListItem> = state
        .buffers
        .iter()
        .enumerate()
        .skip(start)
        .take(end - start)
        .map(|(i, buffer)| {
            let filename = buffer
                .file_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| buffer.file_path.to_string_lossy().to_string());

            let marker = if i == selected { "> " } else { "  " };
            let active_marker = if i == state.active_buffer { " *" } else { "" };

            let style = if i == selected {
                Style::default()
                    .fg(theme.outline_selected)
                    .add_modifier(Modifier::BOLD)
            } else if i == state.active_buffer {
                Style::default().fg(theme.heading_1)
            } else {
                Style::default().fg(theme.text)
            };

            ListItem::new(format!(
                "{}[{}] {}{}",
                marker,
                i + 1,
                filename,
                active_marker
            ))
            .style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Draw the history overlay
fn draw_history_overlay(frame: &mut Frame, state: &AppState) {
    let theme = &state.theme;
    let area = frame.area();

    // Calculate centered area
    let width = 60.min(area.width.saturating_sub(4));
    let height = 15.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let popup_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Draw block
    let entry_count = state.history.entries().len();
    let block = Block::default()
        .title(format!(" History ({}) ", entry_count))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    if state.history.entries().is_empty() {
        let empty = Paragraph::new("No history yet").style(Style::default().fg(theme.empty_state));
        frame.render_widget(empty, inner);
        return;
    }

    // Calculate visible range (scrolling if needed)
    let visible_height = inner.height as usize;
    let selected = state.history_selected;
    let total = state.history.entries().len();

    let start = if selected >= visible_height {
        selected - visible_height + 1
    } else {
        0
    };
    let end = (start + visible_height).min(total);

    // History list
    let items: Vec<ListItem> = state
        .history
        .entries()
        .iter()
        .enumerate()
        .skip(start)
        .take(end - start)
        .map(|(i, entry)| {
            let marker = if i == selected { "> " } else { "  " };
            let icon = if entry.is_url { "🌐 " } else { "📄 " };

            let style = if i == selected {
                Style::default()
                    .fg(theme.outline_selected)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text)
            };

            ListItem::new(format!("{}{}{}", marker, icon, entry.display_name)).style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Draw the bookmarks overlay
fn draw_bookmarks_overlay(frame: &mut Frame, state: &AppState) {
    let theme = &state.theme;
    let area = frame.area();

    // Calculate centered area
    let width = 60.min(area.width.saturating_sub(4));
    let height = 15.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let popup_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, popup_area);

    // Draw block
    let entry_count = state.bookmarks.entries().len();
    let block = Block::default()
        .title(format!(" Bookmarks ({}) - d:delete ", entry_count))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    if state.bookmarks.entries().is_empty() {
        let empty = Paragraph::new("No bookmarks yet (M to add)")
            .style(Style::default().fg(theme.empty_state));
        frame.render_widget(empty, inner);
        return;
    }

    // Calculate visible range (scrolling if needed)
    let visible_height = inner.height as usize;
    let selected = state.bookmarks_selected;
    let total = state.bookmarks.entries().len();

    let start = if selected >= visible_height {
        selected - visible_height + 1
    } else {
        0
    };
    let end = (start + visible_height).min(total);

    // Bookmarks list
    let items: Vec<ListItem> = state
        .bookmarks
        .entries()
        .iter()
        .enumerate()
        .skip(start)
        .take(end - start)
        .map(|(i, bookmark)| {
            let marker = if i == selected { "> " } else { "  " };
            let icon = if bookmark.is_url { "🌐 " } else { "📄 " };

            let style = if i == selected {
                Style::default()
                    .fg(theme.outline_selected)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text)
            };

            ListItem::new(format!("{}{}{}", marker, icon, bookmark.name)).style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Draw the URL input bar
fn draw_url_input(frame: &mut Frame, state: &mut AppState) {
    let theme = &state.theme;
    let area = frame.area();

    // URL input at the bottom of the screen
    let input_area = Rect::new(0, area.height.saturating_sub(3), area.width, 3);

    // Clear background
    frame.render_widget(Clear, input_area);

    // Draw block
    let block = Block::default()
        .title(" Enter URL ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(input_area);
    frame.render_widget(block, input_area);

    // Render textarea
    frame.render_widget(&state.url_textarea, inner);
}

/// Draw the bookmark name input bar
fn draw_bookmark_name_input(frame: &mut Frame, state: &mut AppState) {
    let theme = &state.theme;
    let area = frame.area();

    // Input at the bottom of the screen
    let input_area = Rect::new(0, area.height.saturating_sub(3), area.width, 3);

    // Clear background
    frame.render_widget(Clear, input_area);

    // Draw block
    let block = Block::default()
        .title(" Bookmark Name ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.help_border))
        .style(Style::default().bg(theme.help_bg));

    let inner = block.inner(input_area);
    frame.render_widget(block, input_area);

    // Render textarea
    frame.render_widget(&state.bookmark_name_textarea, inner);
}
