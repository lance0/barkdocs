mod app;
mod config;
mod github;
mod input;
mod markdown;
mod storage;
mod theme;
mod ui;

use anyhow::Result;
use app::AppState;
use config::Config;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Handle --help, --version
    if args.len() >= 2 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--version" | "-V" => {
                println!("barkdocs {}", VERSION);
                return Ok(());
            }
            _ => {}
        }
    }

    // Load config
    let config = Config::load();
    let mut state = AppState::new(&config);

    // Determine what to open (file or URL)
    if args.len() >= 2 && !args[1].starts_with('-') {
        let arg = &args[1];

        // Check if it's a URL
        if arg.starts_with("http://") || arg.starts_with("https://") {
            if let Err(e) = state.load_url(arg) {
                state.status_message = Some(format!("Error loading URL: {}", e));
            }
        } else {
            // Local file path
            let path = PathBuf::from(arg);
            if let Err(e) = state.load_file(&path) {
                state.status_message = Some(format!("Error loading file: {}", e));
            }
        }
    } else {
        // Try README.md in current directory
        let candidates = ["README.md", "readme.md", "README.MD", "Readme.md"];
        let readme = candidates.iter().map(PathBuf::from).find(|p| p.exists());

        if let Some(path) = readme {
            if let Err(e) = state.load_file(&path) {
                state.status_message = Some(format!("Error loading file: {}", e));
            }
        } else {
            // No README found - open file picker
            state.open_file_picker();
        }
    }

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Hide cursor
    terminal.hide_cursor()?;

    // Panic hook for cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic);
    }));

    // Event loop
    let result = run_event_loop(&mut terminal, &mut state);

    // Save history before cleanup
    state.save_history();

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> Result<()> {
    loop {
        // Draw
        terminal.draw(|frame| ui::draw(frame, state))?;

        // Calculate page size
        let page_size = terminal.size()?.height.saturating_sub(4) as usize;

        // Wait for event with timeout (for smooth UI)
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    input::handle_key(state, key, page_size);
                }
                Event::Mouse(mouse) => {
                    input::handle_mouse(state, mouse, page_size);
                }
                Event::Resize(_, _) => {
                    // Terminal handles redraw automatically
                }
                _ => {}
            }
        }

        // Check for file changes (live reload)
        state.check_file_changed();

        if state.should_quit {
            break;
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        r#"barkdocs - A keyboard-driven TUI markdown viewer

USAGE:
    barkdocs [OPTIONS] [FILE|URL]

ARGS:
    [FILE|URL]  Markdown file or GitHub URL to view (defaults to README.md)

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

NAVIGATION:
    j/k, ↑/↓         Scroll up/down
    h/l, ←/→         Scroll left/right (when wrap off)
    g/G              Go to top/bottom
    Ctrl+u/d         Half page up/down
    /                Start search
    n/N              Next/prev search match
    b                Toggle outline panel
    w                Toggle line wrap
    Ctrl+W,v         Split vertical
    Ctrl+W,s         Split horizontal
    Ctrl+W,q         Close pane
    Tab              Switch focus
    ?                Show help
    S                Settings
    q                Quit

URL & HISTORY:
    O                Open URL (GitHub, raw markdown)
    H                View history (recently opened)
    m                View bookmarks
    M                Add current document to bookmarks

SUPPORTED URLS:
    github.com/user/repo              Fetches README.md
    github.com/user/repo/blob/...     Converts to raw URL
    raw.githubusercontent.com/...     Direct raw content

CONFIG:
    ~/.config/barkdocs/config.toml

DATA:
    ~/.local/share/barkdocs/history.json
    ~/.local/share/barkdocs/bookmarks.json

ENVIRONMENT:
    BARKDOCS_THEME        Override theme
    BARKDOCS_LINE_WRAP    Override line wrap (1/0)
"#
    );
}
