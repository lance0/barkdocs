use crate::config::Config;
use crate::markdown::Document;
use crate::theme::Theme;
use ratatui::layout::Rect;
use std::path::PathBuf;
use tui_textarea::TextArea;

/// Input mode for the application
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Search,
    SplitCommand,
}

/// Split direction for panes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SplitDirection {
    #[default]
    None,
    Vertical,
    Horizontal,
}

/// Which panel is focused
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FocusedPanel {
    #[default]
    Content,
    Outline,
}

/// Search match location
#[derive(Clone, Debug)]
pub struct SearchMatch {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

/// State for a single pane
#[derive(Clone)]
pub struct PaneState {
    /// Vertical scroll position (line index)
    pub scroll: usize,
    /// Horizontal scroll position (when wrap off)
    pub horizontal_scroll: usize,
    /// Search query for this pane
    pub search_query: String,
    /// Whether search is regex
    pub search_is_regex: bool,
    /// Search matches
    pub search_matches: Vec<SearchMatch>,
    /// Current match index
    pub current_match: usize,
    /// Textarea for search input
    pub search_textarea: TextArea<'static>,
}

impl Default for PaneState {
    fn default() -> Self {
        Self {
            scroll: 0,
            horizontal_scroll: 0,
            search_query: String::new(),
            search_is_regex: false,
            search_matches: Vec::new(),
            current_match: 0,
            search_textarea: TextArea::default(),
        }
    }
}

impl PaneState {
    /// Create a new pane state
    pub fn new() -> Self {
        Self::default()
    }

    /// Clone for split (same position, clear search)
    pub fn clone_for_split(&self) -> Self {
        Self {
            scroll: self.scroll,
            horizontal_scroll: self.horizontal_scroll,
            search_query: String::new(),
            search_is_regex: false,
            search_matches: Vec::new(),
            current_match: 0,
            search_textarea: TextArea::default(),
        }
    }
}

/// Main application state
pub struct AppState {
    // Document
    pub document: Option<Document>,
    pub file_path: Option<PathBuf>,

    // Rendered lines (cached)
    pub rendered_lines: Vec<ratatui::text::Line<'static>>,

    // Pane management
    pub panes: Vec<PaneState>,
    pub active_pane: usize,
    pub split_direction: SplitDirection,

    // UI state
    pub mode: InputMode,
    pub focused_panel: FocusedPanel,
    pub show_outline: bool,
    pub should_quit: bool,
    pub status_message: Option<String>,
    pub show_help: bool,
    pub show_settings: bool,

    // Display preferences
    pub line_wrap: bool,
    pub show_line_numbers: bool,
    pub theme: Theme,

    // Outline state
    pub outline_selected: usize,

    // Settings overlay state
    pub settings_selected: usize,

    // File picker state
    pub show_file_picker: bool,
    pub file_picker_files: Vec<PathBuf>,
    pub file_picker_selected: usize,

    // Layout tracking (for mouse)
    pub content_areas: Vec<Rect>,
    pub outline_area: Rect,
}

impl AppState {
    /// Create new app state from config
    pub fn new(config: &Config) -> Self {
        Self {
            document: None,
            file_path: None,
            rendered_lines: Vec::new(),

            panes: vec![PaneState::new()],
            active_pane: 0,
            split_direction: SplitDirection::None,

            mode: InputMode::Normal,
            focused_panel: FocusedPanel::Content,
            show_outline: config.show_outline,
            should_quit: false,
            status_message: None,
            show_help: false,
            show_settings: false,

            line_wrap: config.line_wrap,
            show_line_numbers: config.show_line_numbers,
            theme: config.get_theme(),

            outline_selected: 0,
            settings_selected: 0,

            show_file_picker: false,
            file_picker_files: Vec::new(),
            file_picker_selected: 0,

            content_areas: Vec::new(),
            outline_area: Rect::default(),
        }
    }

    /// Load a markdown file
    pub fn load_file(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let document = Document::parse(&content);

        // Pre-render lines
        self.rendered_lines = document.render(&self.theme);

        self.document = Some(document);
        self.file_path = Some(path.to_path_buf());

        // Reset pane state
        for pane in &mut self.panes {
            pane.scroll = 0;
            pane.horizontal_scroll = 0;
            pane.search_matches.clear();
        }

        self.outline_selected = 0;
        self.status_message = None;

        Ok(())
    }

    /// Re-render document (e.g., after theme change)
    pub fn rerender(&mut self) {
        if let Some(doc) = &self.document {
            self.rendered_lines = doc.render(&self.theme);
        }
    }

    /// Get current pane
    pub fn current_pane(&self) -> &PaneState {
        &self.panes[self.active_pane]
    }

    /// Get current pane mutably
    pub fn current_pane_mut(&mut self) -> &mut PaneState {
        &mut self.panes[self.active_pane]
    }

    /// Total line count
    pub fn line_count(&self) -> usize {
        self.rendered_lines.len()
    }

    // === Navigation ===

    /// Scroll down one line
    pub fn scroll_down(&mut self) {
        let max_scroll = self.line_count().saturating_sub(1);
        let pane = self.current_pane_mut();
        pane.scroll = pane.scroll.saturating_add(1).min(max_scroll);
    }

    /// Scroll up one line
    pub fn scroll_up(&mut self) {
        let pane = self.current_pane_mut();
        pane.scroll = pane.scroll.saturating_sub(1);
    }

    /// Scroll down half page
    pub fn scroll_page_down(&mut self, page_size: usize) {
        let half_page = page_size / 2;
        let max_scroll = self.line_count().saturating_sub(1);
        let pane = self.current_pane_mut();
        pane.scroll = pane.scroll.saturating_add(half_page).min(max_scroll);
    }

    /// Scroll up half page
    pub fn scroll_page_up(&mut self, page_size: usize) {
        let half_page = page_size / 2;
        let pane = self.current_pane_mut();
        pane.scroll = pane.scroll.saturating_sub(half_page);
    }

    /// Go to top of document
    pub fn go_to_top(&mut self) {
        self.current_pane_mut().scroll = 0;
    }

    /// Go to bottom of document
    pub fn go_to_bottom(&mut self) {
        let max_scroll = self.line_count().saturating_sub(1);
        self.current_pane_mut().scroll = max_scroll;
    }

    /// Scroll left
    pub fn scroll_left(&mut self) {
        if !self.line_wrap {
            let pane = self.current_pane_mut();
            pane.horizontal_scroll = pane.horizontal_scroll.saturating_sub(4);
        }
    }

    /// Scroll right
    pub fn scroll_right(&mut self) {
        if !self.line_wrap {
            let pane = self.current_pane_mut();
            pane.horizontal_scroll = pane.horizontal_scroll.saturating_add(4);
        }
    }

    /// Go to specific line
    pub fn go_to_line(&mut self, line: usize) {
        let max_scroll = self.line_count().saturating_sub(1);
        self.current_pane_mut().scroll = line.min(max_scroll);
    }

    // === Outline navigation ===

    /// Move outline selection up
    pub fn outline_up(&mut self) {
        if self.outline_selected > 0 {
            self.outline_selected -= 1;
        }
    }

    /// Move outline selection down
    pub fn outline_down(&mut self) {
        if let Some(doc) = &self.document {
            let max = doc.headings.len().saturating_sub(1);
            if self.outline_selected < max {
                self.outline_selected += 1;
            }
        }
    }

    /// Jump to selected heading
    pub fn jump_to_heading(&mut self) {
        if let Some(doc) = &self.document {
            if let Some(heading) = doc.headings.get(self.outline_selected) {
                self.go_to_line(heading.line_number);
                // Switch focus back to content
                self.focused_panel = FocusedPanel::Content;
            }
        }
    }

    // === Search ===

    /// Start search mode
    pub fn start_search(&mut self) {
        self.mode = InputMode::Search;
        let pane = self.current_pane_mut();
        pane.search_textarea = TextArea::default();
    }

    /// Apply search and find matches
    pub fn apply_search(&mut self) {
        let query = self.current_pane().search_textarea.lines().join("");
        if query.is_empty() {
            self.mode = InputMode::Normal;
            return;
        }

        let is_regex = self.current_pane().search_is_regex;
        let mut matches = Vec::new();

        // Build regex or literal pattern
        let pattern = if is_regex {
            match regex::Regex::new(&query) {
                Ok(r) => Some(r),
                Err(_) => {
                    self.status_message = Some("Invalid regex".to_string());
                    self.mode = InputMode::Normal;
                    return;
                }
            }
        } else {
            regex::Regex::new(&regex::escape(&query)).ok()
        };

        if let Some(re) = pattern {
            for (line_idx, line) in self.rendered_lines.iter().enumerate() {
                let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
                for m in re.find_iter(&text) {
                    matches.push(SearchMatch {
                        line: line_idx,
                        start: m.start(),
                        end: m.end(),
                    });
                }
            }
        }

        let match_count = matches.len();
        let pane = self.current_pane_mut();
        pane.search_query = query;
        pane.search_matches = matches;
        pane.current_match = 0;

        self.mode = InputMode::Normal;
        self.status_message = Some(format!("{} matches found", match_count));

        // Jump to first match
        if match_count > 0 {
            self.jump_to_current_match();
        }
    }

    /// Cancel search
    pub fn cancel_search(&mut self) {
        self.mode = InputMode::Normal;
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        let pane = self.current_pane_mut();
        pane.search_query.clear();
        pane.search_matches.clear();
        pane.current_match = 0;
        self.status_message = None;
    }

    /// Next search match
    pub fn next_match(&mut self) {
        let pane = self.current_pane_mut();
        if !pane.search_matches.is_empty() {
            pane.current_match = (pane.current_match + 1) % pane.search_matches.len();
        }
        self.jump_to_current_match();
    }

    /// Previous search match
    pub fn prev_match(&mut self) {
        let pane = self.current_pane_mut();
        if !pane.search_matches.is_empty() {
            pane.current_match = if pane.current_match == 0 {
                pane.search_matches.len() - 1
            } else {
                pane.current_match - 1
            };
        }
        self.jump_to_current_match();
    }

    /// Jump viewport to current match
    fn jump_to_current_match(&mut self) {
        let pane = &self.panes[self.active_pane];
        if let Some(m) = pane.search_matches.get(pane.current_match) {
            let line = m.line;
            self.go_to_line(line);
        }
    }

    /// Toggle regex search
    pub fn toggle_regex(&mut self) {
        self.current_pane_mut().search_is_regex = !self.current_pane().search_is_regex;
    }

    // === Split view ===

    /// Split vertically
    pub fn split_vertical(&mut self) {
        if self.split_direction == SplitDirection::None {
            let new_pane = self.current_pane().clone_for_split();
            self.panes.push(new_pane);
            self.split_direction = SplitDirection::Vertical;
            self.active_pane = 1;
        }
    }

    /// Split horizontally
    pub fn split_horizontal(&mut self) {
        if self.split_direction == SplitDirection::None {
            let new_pane = self.current_pane().clone_for_split();
            self.panes.push(new_pane);
            self.split_direction = SplitDirection::Horizontal;
            self.active_pane = 1;
        }
    }

    /// Close current pane
    pub fn close_pane(&mut self) {
        if self.panes.len() > 1 {
            self.panes.remove(self.active_pane);
            self.split_direction = SplitDirection::None;
            self.active_pane = 0;
        }
    }

    /// Cycle to next pane
    pub fn cycle_pane(&mut self) {
        if self.panes.len() > 1 {
            self.active_pane = (self.active_pane + 1) % self.panes.len();
        }
    }

    // === Display toggles ===

    /// Toggle outline panel
    pub fn toggle_outline(&mut self) {
        self.show_outline = !self.show_outline;
        if !self.show_outline && self.focused_panel == FocusedPanel::Outline {
            self.focused_panel = FocusedPanel::Content;
        }
    }

    /// Toggle line wrap
    pub fn toggle_line_wrap(&mut self) {
        self.line_wrap = !self.line_wrap;
    }

    /// Toggle line numbers
    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    /// Cycle focus between panels
    pub fn cycle_focus(&mut self) {
        if self.show_outline {
            self.focused_panel = match self.focused_panel {
                FocusedPanel::Content => FocusedPanel::Outline,
                FocusedPanel::Outline => FocusedPanel::Content,
            };
        }
    }

    /// Cycle to next theme
    pub fn cycle_theme(&mut self) {
        let themes = Theme::available_themes();
        let current_idx = themes
            .iter()
            .position(|&t| t == self.theme.name)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % themes.len();
        self.theme = Theme::by_name(themes[next_idx]);
        self.rerender();
    }

    // === Clipboard ===

    /// Yank current line to clipboard
    pub fn yank_line(&mut self) {
        let pane = self.current_pane();
        if let Some(line) = self.rendered_lines.get(pane.scroll) {
            let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if clipboard.set_text(&text).is_ok() {
                    self.status_message = Some("Line copied to clipboard".to_string());
                } else {
                    self.status_message = Some("Failed to copy".to_string());
                }
            }
        }
    }

    // === Config ===

    /// Save current settings to config file
    pub fn save_config(&mut self) {
        let config = Config {
            theme: self.theme.name.to_string(),
            line_wrap: self.line_wrap,
            show_outline: self.show_outline,
            outline_width: 24, // default
            show_line_numbers: self.show_line_numbers,
        };

        match config.save() {
            Ok(()) => {
                self.status_message = Some("Settings saved".to_string());
            }
            Err(e) => {
                self.status_message = Some(format!("Failed to save: {}", e));
            }
        }
    }

    // === File Picker ===

    /// Open the file picker, scanning current directory for .md files
    pub fn open_file_picker(&mut self) {
        self.scan_directory(".");
        if !self.file_picker_files.is_empty() {
            self.show_file_picker = true;
            self.file_picker_selected = 0;
        } else {
            self.status_message = Some("No markdown files found".to_string());
        }
    }

    /// Scan a directory for markdown files
    pub fn scan_directory(&mut self, path: &str) {
        self.file_picker_files.clear();

        if let Ok(entries) = std::fs::read_dir(path) {
            let mut files: Vec<PathBuf> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.is_file()
                        && p.extension()
                            .map(|ext| ext.eq_ignore_ascii_case("md"))
                            .unwrap_or(false)
                })
                .collect();

            // Sort alphabetically, but put README files first
            files.sort_by(|a, b| {
                let a_is_readme = a
                    .file_name()
                    .map(|n| n.to_string_lossy().to_lowercase().starts_with("readme"))
                    .unwrap_or(false);
                let b_is_readme = b
                    .file_name()
                    .map(|n| n.to_string_lossy().to_lowercase().starts_with("readme"))
                    .unwrap_or(false);

                match (a_is_readme, b_is_readme) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.cmp(b),
                }
            });

            self.file_picker_files = files;
        }
    }

    /// Move file picker selection up
    pub fn file_picker_up(&mut self) {
        if self.file_picker_selected > 0 {
            self.file_picker_selected -= 1;
        }
    }

    /// Move file picker selection down
    pub fn file_picker_down(&mut self) {
        if !self.file_picker_files.is_empty() {
            let max = self.file_picker_files.len() - 1;
            if self.file_picker_selected < max {
                self.file_picker_selected += 1;
            }
        }
    }

    /// Open the selected file from the picker
    pub fn open_selected_file(&mut self) {
        if let Some(path) = self.file_picker_files.get(self.file_picker_selected).cloned() {
            self.show_file_picker = false;
            if let Err(e) = self.load_file(&path) {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    /// Close the file picker
    pub fn close_file_picker(&mut self) {
        self.show_file_picker = false;
    }

    // === Link Navigation ===

    /// Follow the link on the current line (if any)
    pub fn follow_link(&mut self) {
        let current_line = self.current_pane().scroll;

        let link_url = if let Some(doc) = &self.document {
            doc.link_at_line(current_line).map(|l| l.url.clone())
        } else {
            None
        };

        if let Some(url) = link_url {
            self.open_link(&url);
        } else {
            self.status_message = Some("No link on this line".to_string());
        }
    }

    /// Open a link URL
    fn open_link(&mut self, url: &str) {
        // Check if it's a local markdown file
        if url.ends_with(".md") || url.ends_with(".MD") {
            // Resolve relative to current file's directory
            let path = if let Some(current_path) = &self.file_path {
                if let Some(parent) = current_path.parent() {
                    parent.join(url)
                } else {
                    PathBuf::from(url)
                }
            } else {
                PathBuf::from(url)
            };

            if path.exists() {
                if let Err(e) = self.load_file(&path) {
                    self.status_message = Some(format!("Error opening {}: {}", url, e));
                }
            } else {
                self.status_message = Some(format!("File not found: {}", path.display()));
            }
        } else if url.starts_with("http://") || url.starts_with("https://") {
            // External URL - copy to clipboard
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if clipboard.set_text(url).is_ok() {
                    self.status_message = Some(format!("URL copied: {}", url));
                } else {
                    self.status_message = Some(format!("Link: {}", url));
                }
            } else {
                self.status_message = Some(format!("Link: {}", url));
            }
        } else if url.starts_with('#') {
            // Anchor link - try to find heading
            let anchor = &url[1..];
            let heading_info = if let Some(doc) = &self.document {
                // Find heading that matches anchor (simplified slug matching)
                let target = anchor.to_lowercase().replace('-', " ");
                doc.headings.iter().find(|h| {
                    h.text.to_lowercase() == target
                        || h.text.to_lowercase().replace(' ', "-") == anchor.to_lowercase()
                }).map(|h| (h.line_number, h.text.clone()))
            } else {
                None
            };

            if let Some((line, text)) = heading_info {
                self.go_to_line(line);
                self.status_message = Some(format!("Jumped to: {}", text));
            } else {
                self.status_message = Some(format!("Anchor not found: {}", anchor));
            }
        } else {
            self.status_message = Some(format!("Unknown link type: {}", url));
        }
    }
}
