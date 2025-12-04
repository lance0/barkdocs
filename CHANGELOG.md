# Changelog

All notable changes to barkdocs will be documented in this file.

## [1.1.4] - 2025-12-04

### Added
- Resizable outline panel (`<`/`>` or `[`/`]` keys, or `h`/`l` when focused)
- Syntax highlighting and auto-reload settings now persist to config
- Environment variable overrides: `BARKDOCS_SYNTAX_HIGHLIGHTING`, `BARKDOCS_AUTO_RELOAD`

### Fixed
- **Outline navigation** - Jumping to headings now goes to correct line position
- **Buffer URL state** - Switching buffers properly preserves/restores URL vs file state
- **Auto-reload per-buffer** - File modification times tracked per-buffer, not globally
- **List item links** - Links in list items now work correctly (not just first item)
- **Non-blocking URL fetch** - URL loading no longer freezes the UI (async with thread)
- **Settings save** - Outline width now saved correctly (was hardcoded to 24)
- Long heading text in outline panel now truncates with ellipsis

### Changed
- URL fetching moved to background thread for responsive UI during network requests

## [1.1.3] - 2025-12-04

### Added
- Screenshot in README
- CI/CD pipeline with GitHub Actions
- Automated releases and crates.io publishing
- 40 unit tests for core functionality

### Fixed
- Version number now correctly set in Cargo.toml
- Fixed crates.io publishing

## [1.0.0] - 2025-12-04

### Added

#### Core Features
- Markdown rendering with pulldown-cmark
- Vim-style navigation (j/k, g/G, Ctrl+u/d)
- Search with `/`, navigate matches with n/N
- Regex search support (Ctrl+r to toggle)
- Outline panel with heading navigation (toggle with `b`)
- Line wrap toggle (`w`)
- Line numbers toggle (`#`)

#### GitHub URL Support
- Open markdown files directly from GitHub URLs
- Support for repo URLs (`github.com/user/repo` fetches README)
- Support for blob URLs (converts to raw automatically)
- Support for raw.githubusercontent.com URLs
- Symlink detection and following (for repos like Next.js)
- Branch detection: HEAD, main, master, canary, develop, dev, trunk
- Session caching for fetched URLs
- Follow GitHub links in documents with Enter/f

#### History & Bookmarks
- History tracking for all opened files and URLs (`H` to view)
- Bookmarks for favorite documents (`m` to view, `M` to add)
- Persistent storage in `~/.local/share/barkdocs/`

#### Multi-Document Support
- Multiple buffer support (open several files)
- Buffer list (`B`) with quick switching
- Buffer navigation (Ctrl+n/p for next/prev, Ctrl+x to close)
- Split view - vertical (`Ctrl+W,v`) and horizontal (`Ctrl+W,s`)
- Pane cycling (`Ctrl+W,w`) and closing (`Ctrl+W,q`)

#### Syntax Highlighting
- Code block syntax highlighting via syntect
- Toggle with Ctrl+s
- Supports common languages

#### Themes
- 11 built-in color themes
- default, dracula, gruvbox, nord, solarized-dark, solarized-light
- monokai, catppuccin, tokyo-night, onedark, matrix
- Theme cycling in settings (`S`)

#### Configuration
- Config file at `~/.config/barkdocs/config.toml`
- Persistent settings (theme, line wrap, outline, etc.)
- Environment variable overrides (BARKDOCS_THEME, BARKDOCS_LINE_WRAP)

#### User Interface
- Settings overlay (`S`) for runtime configuration
- Help overlay (`?`) with all keybindings
- File picker (`o`) for opening local files
- URL input prompt (`O`) for opening remote files
- Status bar with mode, position, and active flags
- Live file reload (toggle with `R`)

#### Other
- Yank/copy current line (`y`)
- Mouse support (scroll, click to focus)
- Link following for local markdown files
- Anchor link navigation within documents
- Panic hook for clean terminal restoration
