use crate::config::Config;
use crate::github::GitHubFetcher;
use crate::markdown::{Document, SyntaxHighlighter};
use crate::storage::{Bookmarks, History};
use crate::theme::Theme;
use ratatui::layout::Rect;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::SystemTime;
use tui_textarea::TextArea;

/// Result type for async URL fetch: Ok((content, url)) or Err(error_message)
pub type FetchResult = Result<(String, String), String>;

/// Input mode for the application
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Search,
    SplitCommand,
    UrlInput,
    BookmarkName,
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
#[derive(Clone, Default)]
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

/// A document buffer (open file with state)
#[derive(Clone)]
pub struct DocumentBuffer {
    pub document: Document,
    pub file_path: PathBuf,
    pub url: Option<String>,
    pub rendered_lines: Vec<ratatui::text::Line<'static>>,
    pub scroll: usize,
    pub horizontal_scroll: usize,
    pub outline_selected: usize,
    pub modified_time: Option<SystemTime>,
}

/// Main application state
pub struct AppState {
    // Document
    pub document: Option<Document>,
    pub file_path: Option<PathBuf>,

    // Rendered lines (cached)
    pub rendered_lines: Vec<ratatui::text::Line<'static>>,

    // Multiple document buffers
    pub buffers: Vec<DocumentBuffer>,
    pub active_buffer: usize,
    pub show_buffer_list: bool,
    pub buffer_list_selected: usize,

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
    pub syntax_highlighting: bool,
    pub highlighter: SyntaxHighlighter,

    // Outline state
    pub outline_selected: usize,
    pub outline_width: u16,

    // Settings overlay state
    pub settings_selected: usize,

    // File picker state
    pub show_file_picker: bool,
    pub file_picker_files: Vec<PathBuf>,
    pub file_picker_selected: usize,

    // Live reload
    pub auto_reload: bool,
    pub file_modified_time: Option<SystemTime>,

    // URL support
    pub github_fetcher: GitHubFetcher,
    pub current_url: Option<String>,
    pub is_loading: bool,
    pub fetch_receiver: Option<Receiver<FetchResult>>,

    // History & Bookmarks
    pub history: History,
    pub bookmarks: Bookmarks,

    // History overlay state
    pub show_history: bool,
    pub history_selected: usize,

    // Bookmarks overlay state
    pub show_bookmarks: bool,
    pub bookmarks_selected: usize,

    // URL input state
    pub show_url_input: bool,
    pub url_textarea: TextArea<'static>,

    // Bookmark name input state
    pub show_bookmark_name_input: bool,
    pub bookmark_name_textarea: TextArea<'static>,

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

            buffers: Vec::new(),
            active_buffer: 0,
            show_buffer_list: false,
            buffer_list_selected: 0,

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
            syntax_highlighting: config.syntax_highlighting,
            highlighter: SyntaxHighlighter::default(),

            outline_selected: 0,
            outline_width: config.outline_width,
            settings_selected: 0,

            show_file_picker: false,
            file_picker_files: Vec::new(),
            file_picker_selected: 0,

            auto_reload: config.auto_reload,
            file_modified_time: None,

            github_fetcher: GitHubFetcher::new(),
            current_url: None,
            is_loading: false,
            fetch_receiver: None,

            history: History::load(),
            bookmarks: Bookmarks::load(),

            show_history: false,
            history_selected: 0,

            show_bookmarks: false,
            bookmarks_selected: 0,

            show_url_input: false,
            url_textarea: TextArea::default(),

            show_bookmark_name_input: false,
            bookmark_name_textarea: TextArea::default(),

            content_areas: Vec::new(),
            outline_area: Rect::default(),
        }
    }

    /// Load a markdown file
    pub fn load_file(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        // Save current document to buffer before loading new one
        self.save_to_buffer();

        // Check if this file is already in a buffer
        let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if let Some(idx) = self.buffers.iter().position(|b| {
            b.file_path
                .canonicalize()
                .unwrap_or_else(|_| b.file_path.clone())
                == abs_path
        }) {
            // File already open, switch to that buffer
            self.load_from_buffer(idx);
            return Ok(());
        }

        let content = std::fs::read_to_string(path)?;
        let mut document = Document::parse(&content);

        // Pre-render lines with optional syntax highlighting
        let highlighter = if self.syntax_highlighting {
            Some(&self.highlighter)
        } else {
            None
        };
        self.rendered_lines = document.render_with_highlighting(&self.theme, highlighter);

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

        // Store file modification time for auto-reload
        self.file_modified_time = std::fs::metadata(path).ok().and_then(|m| m.modified().ok());

        // Add to buffer list
        self.save_to_buffer();

        // Clear any URL state since we're loading a local file
        self.current_url = None;

        // Add to history
        let display_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        self.history
            .add(&path.to_string_lossy(), false, &display_name);

        Ok(())
    }

    /// Check if the current file has changed and reload if needed
    pub fn check_file_changed(&mut self) -> bool {
        if !self.auto_reload {
            return false;
        }

        let Some(path) = &self.file_path else {
            return false;
        };

        let Ok(metadata) = std::fs::metadata(path) else {
            return false;
        };

        let Ok(current_modified) = metadata.modified() else {
            return false;
        };

        // Check if modification time has changed
        if let Some(stored_time) = self.file_modified_time {
            if current_modified > stored_time {
                // File has been modified, reload it
                let path_clone = path.clone();

                // Read and re-parse the file
                if let Ok(content) = std::fs::read_to_string(&path_clone) {
                    let mut document = Document::parse(&content);

                    // Re-render with current settings
                    let highlighter = if self.syntax_highlighting {
                        Some(&self.highlighter)
                    } else {
                        None
                    };
                    self.rendered_lines =
                        document.render_with_highlighting(&self.theme, highlighter);
                    self.document = Some(document);
                    self.file_modified_time = Some(current_modified);
                    self.status_message = Some("File reloaded".to_string());
                    return true;
                }
            }
        }

        false
    }

    /// Toggle auto-reload
    pub fn toggle_auto_reload(&mut self) {
        self.auto_reload = !self.auto_reload;
        self.status_message = Some(if self.auto_reload {
            "Auto-reload enabled".to_string()
        } else {
            "Auto-reload disabled".to_string()
        });
    }

    /// Re-render document (e.g., after theme change)
    pub fn rerender(&mut self) {
        if let Some(doc) = &mut self.document {
            let highlighter = if self.syntax_highlighting {
                Some(&self.highlighter)
            } else {
                None
            };
            self.rendered_lines = doc.render_with_highlighting(&self.theme, highlighter);
        }
    }

    /// Toggle syntax highlighting
    pub fn toggle_syntax_highlighting(&mut self) {
        self.syntax_highlighting = !self.syntax_highlighting;
        self.rerender();
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
                self.go_to_line(heading.rendered_line);
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
            outline_width: self.outline_width,
            show_line_numbers: self.show_line_numbers,
            syntax_highlighting: self.syntax_highlighting,
            auto_reload: self.auto_reload,
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
        if let Some(path) = self
            .file_picker_files
            .get(self.file_picker_selected)
            .cloned()
        {
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
            // Check if this is a markdown URL we can fetch
            let can_fetch = url.ends_with(".md")
                || url.ends_with(".MD")
                || url.ends_with(".markdown")
                || url.contains("github.com")
                || url.contains("raw.githubusercontent.com");

            if can_fetch {
                // Load the URL (non-blocking)
                self.start_url_fetch(url);
            } else {
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
            }
        } else if let Some(anchor) = url.strip_prefix('#') {
            // Anchor link - try to find heading
            let heading_info = if let Some(doc) = &self.document {
                // Find heading that matches anchor (simplified slug matching)
                let target = anchor.to_lowercase().replace('-', " ");
                doc.headings
                    .iter()
                    .find(|h| {
                        h.text.to_lowercase() == target
                            || h.text.to_lowercase().replace(' ', "-") == anchor.to_lowercase()
                    })
                    .map(|h| (h.line_number, h.text.clone()))
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

    // === Buffer Management ===

    /// Save current document state to its buffer
    fn save_to_buffer(&mut self) {
        if self.document.is_none() || self.file_path.is_none() {
            return;
        }

        let doc = self.document.as_ref().unwrap();
        let path = self.file_path.as_ref().unwrap();
        let pane = self.current_pane();

        let buffer = DocumentBuffer {
            document: doc.clone(),
            file_path: path.clone(),
            url: self.current_url.clone(),
            rendered_lines: self.rendered_lines.clone(),
            scroll: pane.scroll,
            horizontal_scroll: pane.horizontal_scroll,
            outline_selected: self.outline_selected,
            modified_time: self.file_modified_time,
        };

        // Check if buffer already exists for this file
        // Skip URL buffers when searching by path to avoid false matches
        if let Some(idx) = self.buffers.iter().position(|b| {
            b.url.is_none() && b.file_path == *path
        }) {
            self.buffers[idx] = buffer;
            self.active_buffer = idx;
        } else {
            self.buffers.push(buffer);
            self.active_buffer = self.buffers.len() - 1;
        }
    }

    /// Load a buffer into the current view
    fn load_from_buffer(&mut self, index: usize) {
        if index >= self.buffers.len() {
            return;
        }

        // First save current document state
        self.save_to_buffer();

        // Clone all values we need from the buffer first
        let buffer = &self.buffers[index];
        let document = buffer.document.clone();
        let file_path = buffer.file_path.clone();
        let url = buffer.url.clone();
        let rendered_lines = buffer.rendered_lines.clone();
        let outline_selected = buffer.outline_selected;
        let scroll = buffer.scroll;
        let horizontal_scroll = buffer.horizontal_scroll;
        let modified_time = buffer.modified_time;

        // Now apply them
        self.document = Some(document);
        self.file_path = Some(file_path);
        self.current_url = url;
        self.rendered_lines = rendered_lines;
        self.outline_selected = outline_selected;
        self.file_modified_time = modified_time;
        self.is_loading = false;

        // Restore scroll position
        let pane = self.current_pane_mut();
        pane.scroll = scroll;
        pane.horizontal_scroll = horizontal_scroll;

        self.active_buffer = index;
    }

    /// Open buffer list overlay
    pub fn open_buffer_list(&mut self) {
        if !self.buffers.is_empty() {
            self.show_buffer_list = true;
            self.buffer_list_selected = self.active_buffer;
        } else {
            self.status_message = Some("No buffers open".to_string());
        }
    }

    /// Close buffer list overlay
    pub fn close_buffer_list(&mut self) {
        self.show_buffer_list = false;
    }

    /// Move buffer list selection up
    pub fn buffer_list_up(&mut self) {
        if self.buffer_list_selected > 0 {
            self.buffer_list_selected -= 1;
        }
    }

    /// Move buffer list selection down
    pub fn buffer_list_down(&mut self) {
        if !self.buffers.is_empty() && self.buffer_list_selected < self.buffers.len() - 1 {
            self.buffer_list_selected += 1;
        }
    }

    /// Switch to selected buffer
    pub fn select_buffer(&mut self) {
        let idx = self.buffer_list_selected;
        self.load_from_buffer(idx);
        self.show_buffer_list = false;
        self.status_message = Some(format!("Switched to buffer {}", idx + 1));
    }

    /// Go to next buffer
    pub fn next_buffer(&mut self) {
        if self.buffers.is_empty() {
            return;
        }
        self.save_to_buffer();
        let next = (self.active_buffer + 1) % self.buffers.len();
        self.load_from_buffer(next);
        self.status_message = Some(format!("Buffer {}/{}", next + 1, self.buffers.len()));
    }

    /// Go to previous buffer
    pub fn prev_buffer(&mut self) {
        if self.buffers.is_empty() {
            return;
        }
        self.save_to_buffer();
        let prev = if self.active_buffer == 0 {
            self.buffers.len() - 1
        } else {
            self.active_buffer - 1
        };
        self.load_from_buffer(prev);
        self.status_message = Some(format!("Buffer {}/{}", prev + 1, self.buffers.len()));
    }

    /// Close current buffer
    pub fn close_buffer(&mut self) {
        if self.buffers.len() <= 1 {
            self.status_message = Some("Cannot close last buffer".to_string());
            return;
        }

        self.buffers.remove(self.active_buffer);
        if self.active_buffer >= self.buffers.len() {
            self.active_buffer = self.buffers.len() - 1;
        }
        self.load_from_buffer(self.active_buffer);
        self.status_message = Some(format!("{} buffers remaining", self.buffers.len()));
    }

    /// Get buffer count
    #[allow(dead_code)]
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }

    // === URL Loading ===

    /// Load content from a URL
    /// Start an async URL fetch (non-blocking)
    pub fn start_url_fetch(&mut self, url: &str) {
        self.is_loading = true;
        self.status_message = Some(format!("Loading {}...", url));

        let (tx, rx) = std::sync::mpsc::channel();
        self.fetch_receiver = Some(rx);

        let fetcher = self.github_fetcher.clone();
        let url_owned = url.to_string();

        std::thread::spawn(move || {
            let result = fetcher
                .fetch(&url_owned)
                .map(|content| (content, url_owned.clone()))
                .map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
    }

    /// Check if async fetch is complete and process result
    pub fn check_fetch_complete(&mut self) {
        if let Some(rx) = &self.fetch_receiver {
            match rx.try_recv() {
                Ok(result) => {
                    self.fetch_receiver = None;
                    self.is_loading = false;

                    match result {
                        Ok((content, url)) => {
                            self.finish_load_url(&content, &url);
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    // Sender dropped (thread panicked or finished without sending)
                    self.fetch_receiver = None;
                    self.is_loading = false;
                    self.status_message = Some("Error: fetch failed unexpectedly".to_string());
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    // Still waiting, nothing to do
                }
            }
        }
    }

    /// Finish loading URL content (after fetch completes)
    fn finish_load_url(&mut self, content: &str, url: &str) {
        // Parse and render
        let mut document = Document::parse(content);
        let highlighter = if self.syntax_highlighting {
            Some(&self.highlighter)
        } else {
            None
        };
        self.rendered_lines = document.render_with_highlighting(&self.theme, highlighter);

        // Extract display name from URL
        let display_name = url.rsplit('/').next().unwrap_or(url).to_string();

        // Create a placeholder file path for buffer management
        let placeholder_path = PathBuf::from(format!("[URL] {}", display_name));

        // Update state
        self.document = Some(document);
        self.file_path = Some(placeholder_path);
        self.current_url = Some(url.to_string());
        self.file_modified_time = None; // No auto-reload for URLs

        // Reset pane state
        for pane in &mut self.panes {
            pane.scroll = 0;
            pane.horizontal_scroll = 0;
            pane.search_matches.clear();
        }

        self.outline_selected = 0;

        // Add to history
        self.history.add(url, true, &display_name);
        let _ = self.history.save();

        self.status_message = Some(format!("Loaded: {}", display_name));
    }

    /// Load URL content (blocking - kept for potential future use)
    #[allow(dead_code)]
    pub fn load_url(&mut self, url: &str) -> anyhow::Result<()> {
        self.is_loading = true;
        self.status_message = Some(format!("Loading {}...", url));

        // Fetch content (blocking)
        let result = self.github_fetcher.fetch(url);

        self.is_loading = false;

        let content = result.map_err(|e| anyhow::anyhow!("{}", e))?;

        self.finish_load_url(&content, url);

        Ok(())
    }

    /// Get the current location (file path or URL)
    pub fn current_location(&self) -> Option<String> {
        self.current_url.clone().or_else(|| {
            self.file_path
                .as_ref()
                .map(|path| path.to_string_lossy().to_string())
        })
    }

    /// Check if currently viewing a URL
    pub fn is_viewing_url(&self) -> bool {
        self.current_url.is_some()
    }

    // === URL Input Mode ===

    /// Start URL input mode
    pub fn start_url_input(&mut self) {
        self.mode = InputMode::UrlInput;
        self.show_url_input = true;
        self.url_textarea = TextArea::default();
    }

    /// Submit URL and load it
    pub fn submit_url(&mut self) {
        let url = self.url_textarea.lines().join("");
        self.mode = InputMode::Normal;
        self.show_url_input = false;

        if url.is_empty() {
            return;
        }

        // Use non-blocking fetch
        self.start_url_fetch(&url);
    }

    /// Cancel URL input
    pub fn cancel_url_input(&mut self) {
        self.mode = InputMode::Normal;
        self.show_url_input = false;
    }

    // === History ===

    /// Open history overlay
    pub fn open_history(&mut self) {
        if self.history.entries().is_empty() {
            self.status_message = Some("No history".to_string());
            return;
        }
        self.show_history = true;
        self.history_selected = 0;
    }

    /// Close history overlay
    pub fn close_history(&mut self) {
        self.show_history = false;
    }

    /// Move history selection up
    pub fn history_up(&mut self) {
        if self.history_selected > 0 {
            self.history_selected -= 1;
        }
    }

    /// Move history selection down
    pub fn history_down(&mut self) {
        let max = self.history.entries().len().saturating_sub(1);
        if self.history_selected < max {
            self.history_selected += 1;
        }
    }

    /// Open selected history item
    pub fn select_history(&mut self) {
        let entries = self.history.entries();
        if let Some(entry) = entries.get(self.history_selected).cloned() {
            self.show_history = false;

            if entry.is_url {
                // Non-blocking URL fetch
                self.start_url_fetch(&entry.location);
            } else {
                let path = PathBuf::from(&entry.location);
                if let Err(e) = self.load_file(&path) {
                    self.status_message = Some(format!("Error: {}", e));
                }
            }
        }
    }

    // === Bookmarks ===

    /// Open bookmarks overlay
    pub fn open_bookmarks(&mut self) {
        if self.bookmarks.entries().is_empty() {
            self.status_message = Some("No bookmarks".to_string());
            return;
        }
        self.show_bookmarks = true;
        self.bookmarks_selected = 0;
    }

    /// Close bookmarks overlay
    pub fn close_bookmarks(&mut self) {
        self.show_bookmarks = false;
    }

    /// Move bookmarks selection up
    pub fn bookmarks_up(&mut self) {
        if self.bookmarks_selected > 0 {
            self.bookmarks_selected -= 1;
        }
    }

    /// Move bookmarks selection down
    pub fn bookmarks_down(&mut self) {
        let max = self.bookmarks.entries().len().saturating_sub(1);
        if self.bookmarks_selected < max {
            self.bookmarks_selected += 1;
        }
    }

    /// Open selected bookmark
    pub fn select_bookmark(&mut self) {
        let entries = self.bookmarks.entries();
        if let Some(bookmark) = entries.get(self.bookmarks_selected).cloned() {
            self.show_bookmarks = false;

            if bookmark.is_url {
                // Non-blocking URL fetch
                self.start_url_fetch(&bookmark.location);
            } else {
                let path = PathBuf::from(&bookmark.location);
                if let Err(e) = self.load_file(&path) {
                    self.status_message = Some(format!("Error: {}", e));
                }
            }
        }
    }

    /// Start adding a bookmark (show name input)
    pub fn start_add_bookmark(&mut self) {
        let Some(location) = self.current_location() else {
            self.status_message = Some("Nothing to bookmark".to_string());
            return;
        };

        // Check if already bookmarked
        if self.bookmarks.is_bookmarked(&location) {
            self.status_message = Some("Already bookmarked".to_string());
            return;
        }

        // Pre-fill with filename/URL
        let default_name = if let Some(url) = &self.current_url {
            url.rsplit('/').next().unwrap_or(url).to_string()
        } else if let Some(path) = &self.file_path {
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        } else {
            String::new()
        };

        self.mode = InputMode::BookmarkName;
        self.show_bookmark_name_input = true;
        self.bookmark_name_textarea = TextArea::default();
        self.bookmark_name_textarea.insert_str(&default_name);
    }

    /// Confirm adding bookmark with name
    pub fn confirm_add_bookmark(&mut self) {
        let name = self.bookmark_name_textarea.lines().join("");
        self.mode = InputMode::Normal;
        self.show_bookmark_name_input = false;

        let Some(location) = self.current_location() else {
            return;
        };

        let is_url = self.current_url.is_some();
        let name = if name.is_empty() {
            location.rsplit('/').next().unwrap_or(&location).to_string()
        } else {
            name
        };

        self.bookmarks.add(&location, is_url, &name);
        if let Err(e) = self.bookmarks.save() {
            self.status_message = Some(format!("Failed to save bookmark: {}", e));
        } else {
            self.status_message = Some(format!("Bookmarked: {}", name));
        }
    }

    /// Cancel adding bookmark
    pub fn cancel_add_bookmark(&mut self) {
        self.mode = InputMode::Normal;
        self.show_bookmark_name_input = false;
    }

    /// Delete selected bookmark
    pub fn delete_selected_bookmark(&mut self) {
        if self.bookmarks_selected < self.bookmarks.entries().len() {
            self.bookmarks.remove(self.bookmarks_selected);
            let _ = self.bookmarks.save();
            self.status_message = Some("Bookmark deleted".to_string());

            // Adjust selection if needed
            if self.bookmarks_selected > 0
                && self.bookmarks_selected >= self.bookmarks.entries().len()
            {
                self.bookmarks_selected = self.bookmarks.entries().len().saturating_sub(1);
            }

            // Close if no more bookmarks
            if self.bookmarks.entries().is_empty() {
                self.show_bookmarks = false;
            }
        }
    }

    /// Save history (call on exit)
    pub fn save_history(&self) {
        let _ = self.history.save();
    }
}
