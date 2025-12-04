# Changelog

All notable changes to barkdocs will be documented in this file.

## [1.1.1] - 2025-12-04

### Added
- Kimchi (Italian Greyhound) as official mascot
- Screenshot in README
- CI/CD pipeline with GitHub Actions
- Automated releases and crates.io publishing
- 40 unit tests for core functionality

### Fixed
- Version number now correctly set in Cargo.toml

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
