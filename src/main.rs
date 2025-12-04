mod app;
mod config;
mod input;
mod markdown;
mod theme;
mod ui;

use anyhow::Result;
use app::AppState;
use config::Config;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
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

    // Determine file to open
    let file_to_open = if args.len() >= 2 && !args[1].starts_with('-') {
        Some(PathBuf::from(&args[1]))
    } else {
        // Try README.md in current directory
        let candidates = ["README.md", "readme.md", "README.MD", "Readme.md"];
        candidates
            .iter()
            .map(PathBuf::from)
            .find(|p| p.exists())
    };

    // Load file if found, otherwise open file picker
    if let Some(path) = file_to_open {
        if let Err(e) = state.load_file(&path) {
            state.status_message = Some(format!("Error loading file: {}", e));
        }
    } else {
        // No README found - open file picker
        state.open_file_picker();
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

        // Clear transient status messages after a while
        // (In a more complete implementation, we'd track time)

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
    barkdocs [OPTIONS] [FILE]

ARGS:
    [FILE]    Markdown file to view (defaults to README.md)

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

CONFIG:
    ~/.config/barkdocs/config.toml

ENVIRONMENT:
    BARKDOCS_THEME        Override theme
    BARKDOCS_LINE_WRAP    Override line wrap (1/0)
"#
    );
}
