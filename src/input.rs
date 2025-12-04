use crate::app::{AppState, FocusedPanel, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use tui_textarea::Input;

/// Handle keyboard input
pub fn handle_key(state: &mut AppState, key: KeyEvent, page_size: usize) {
    // Handle overlays first
    if state.show_help {
        handle_help_overlay(state, key);
        return;
    }

    if state.show_settings {
        handle_settings_overlay(state, key);
        return;
    }

    if state.show_file_picker {
        handle_file_picker(state, key);
        return;
    }

    match state.mode {
        InputMode::Normal => handle_normal_mode(state, key, page_size),
        InputMode::Search => handle_search_mode(state, key),
        InputMode::SplitCommand => handle_split_command(state, key),
    }
}

/// Handle normal mode input
fn handle_normal_mode(state: &mut AppState, key: KeyEvent, page_size: usize) {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        // Quit
        KeyCode::Char('q') => state.should_quit = true,
        KeyCode::Char('c') if ctrl => state.should_quit = true,

        // Help
        KeyCode::Char('?') => state.show_help = true,

        // Settings
        KeyCode::Char('S') => state.show_settings = true,

        // Outline toggle
        KeyCode::Char('b') => state.toggle_outline(),

        // Panel focus
        KeyCode::Tab => state.cycle_focus(),

        // Vertical navigation
        KeyCode::Char('j') | KeyCode::Down => match state.focused_panel {
            FocusedPanel::Content => state.scroll_down(),
            FocusedPanel::Outline => state.outline_down(),
        },
        KeyCode::Char('k') | KeyCode::Up => match state.focused_panel {
            FocusedPanel::Content => state.scroll_up(),
            FocusedPanel::Outline => state.outline_up(),
        },

        // Horizontal navigation (content only, when wrap off)
        KeyCode::Char('h') | KeyCode::Left => {
            if state.focused_panel == FocusedPanel::Content {
                state.scroll_left();
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if state.focused_panel == FocusedPanel::Content {
                state.scroll_right();
            }
        }

        // Page navigation
        KeyCode::PageDown => state.scroll_page_down(page_size),
        KeyCode::PageUp => state.scroll_page_up(page_size),
        KeyCode::Char('d') if ctrl => state.scroll_page_down(page_size),
        KeyCode::Char('u') if ctrl => state.scroll_page_up(page_size),

        // Top/bottom
        KeyCode::Char('g') => state.go_to_top(),
        KeyCode::Char('G') => state.go_to_bottom(),
        KeyCode::Home => state.go_to_top(),
        KeyCode::End => state.go_to_bottom(),

        // Jump to heading (outline) or follow link (content)
        KeyCode::Enter => {
            if state.focused_panel == FocusedPanel::Outline {
                state.jump_to_heading();
            } else {
                state.follow_link();
            }
        }

        // Follow link (vim-style gf = "go file")
        KeyCode::Char('f') if state.focused_panel == FocusedPanel::Content => {
            state.follow_link();
        }

        // Search
        KeyCode::Char('/') => state.start_search(),
        KeyCode::Char('n') => state.next_match(),
        KeyCode::Char('N') => state.prev_match(),

        // Display toggles
        KeyCode::Char('w') if !ctrl => state.toggle_line_wrap(),
        KeyCode::Char('#') => state.toggle_line_numbers(),

        // Split view (Ctrl+W prefix)
        KeyCode::Char('w') if ctrl => state.mode = InputMode::SplitCommand,

        // Yank
        KeyCode::Char('y') => state.yank_line(),

        // Open file picker
        KeyCode::Char('o') => state.open_file_picker(),

        // Clear search / escape
        KeyCode::Esc => state.clear_search(),

        _ => {}
    }
}

/// Handle search mode input
fn handle_search_mode(state: &mut AppState, key: KeyEvent) {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Enter => state.apply_search(),
        KeyCode::Esc => state.cancel_search(),
        KeyCode::Char('r') if ctrl => state.toggle_regex(),
        _ => {
            // Forward to textarea
            let input = Input::from(key);
            state.current_pane_mut().search_textarea.input(input);
        }
    }
}

/// Handle split command mode (after Ctrl+W)
fn handle_split_command(state: &mut AppState, key: KeyEvent) {
    state.mode = InputMode::Normal;

    match key.code {
        KeyCode::Char('v') => state.split_vertical(),
        KeyCode::Char('s') => state.split_horizontal(),
        KeyCode::Char('q') => state.close_pane(),
        KeyCode::Char('w') => state.cycle_pane(),
        // Could add h/j/k/l for directional focus
        _ => {}
    }
}

/// Handle help overlay input
fn handle_help_overlay(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
            state.show_help = false;
        }
        _ => {}
    }
}

/// Handle settings overlay input
fn handle_settings_overlay(state: &mut AppState, key: KeyEvent) {
    const NUM_SETTINGS: usize = 5; // 4 toggles + Save

    match key.code {
        KeyCode::Esc | KeyCode::Char('S') => {
            state.show_settings = false;
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state.settings_selected = (state.settings_selected + 1) % NUM_SETTINGS;
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.settings_selected = state
                .settings_selected
                .checked_sub(1)
                .unwrap_or(NUM_SETTINGS - 1);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            toggle_setting(state, state.settings_selected);
        }
        _ => {}
    }
}

/// Handle file picker overlay input
fn handle_file_picker(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('o') => {
            state.close_file_picker();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state.file_picker_down();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.file_picker_up();
        }
        KeyCode::Enter => {
            state.open_selected_file();
        }
        KeyCode::Char('q') => {
            state.close_file_picker();
        }
        _ => {}
    }
}

/// Toggle a setting by index
fn toggle_setting(state: &mut AppState, index: usize) {
    match index {
        0 => state.cycle_theme(),
        1 => state.toggle_line_wrap(),
        2 => state.toggle_outline(),
        3 => state.toggle_line_numbers(),
        4 => state.save_config(),
        _ => {}
    }
}

/// Handle mouse input
pub fn handle_mouse(state: &mut AppState, mouse: MouseEvent, _page_size: usize) {
    match mouse.kind {
        MouseEventKind::ScrollUp => {
            for _ in 0..3 {
                state.scroll_up();
            }
        }
        MouseEventKind::ScrollDown => {
            for _ in 0..3 {
                state.scroll_down();
            }
        }
        MouseEventKind::Down(MouseButton::Left) => {
            // Check if click is in outline area
            let x = mouse.column;
            let y = mouse.row;

            if state.show_outline && x < state.outline_area.right() && y >= state.outline_area.y {
                state.focused_panel = FocusedPanel::Outline;
            } else {
                state.focused_panel = FocusedPanel::Content;

                // Check which pane was clicked (for split view)
                for (i, area) in state.content_areas.iter().enumerate() {
                    if x >= area.x && x < area.right() && y >= area.y && y < area.bottom() {
                        state.active_pane = i;
                        break;
                    }
                }
            }
        }
        _ => {}
    }
}
