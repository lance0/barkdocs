# Barkdocs

A keyboard-driven TUI markdown viewer, companion to [barklog](https://github.com/lance0/barklog).

## Features

- **Fast markdown rendering** - Syntax highlighting for code blocks
- **GitHub URL support** - Open READMEs directly from GitHub URLs
- **History & Bookmarks** - Track recently opened files, save favorites
- **Vim-like navigation** - j/k scrolling, search with `/`, regex support
- **Outline panel** - Quick navigation via document headings
- **Split view** - View multiple sections side-by-side
- **Multiple buffers** - Open several documents, switch between them
- **11 color themes** - Match your terminal aesthetic
- **Live reload** - Auto-refresh when files change
- **Configurable** - Persistent settings via config file

## Installation

```bash
cargo install barkdocs
```

Or build from source:

```bash
git clone https://github.com/lance0/barkdocs
cd barkdocs
cargo build --release
```

## Usage

```bash
# View a local file
barkdocs README.md

# Open GitHub repo README
barkdocs https://github.com/vercel/next.js

# Open specific GitHub file
barkdocs https://github.com/rust-lang/rust/blob/master/README.md

# Auto-open README.md in current directory
barkdocs
```

## Keybindings

### Navigation
| Key | Action |
|-----|--------|
| `j/k`, `↑/↓` | Scroll up/down |
| `h/l`, `←/→` | Scroll left/right |
| `g/G` | Go to top/bottom |
| `Ctrl+u/d` | Half page up/down |
| `Enter/f` | Follow link |
| `y` | Yank (copy) current line |

### Search
| Key | Action |
|-----|--------|
| `/` | Start search |
| `n/N` | Next/prev match |
| `Ctrl+r` | Toggle regex mode |

### View
| Key | Action |
|-----|--------|
| `b` | Toggle outline panel |
| `w` | Toggle line wrap |
| `#` | Toggle line numbers |
| `Ctrl+s` | Toggle syntax highlighting |
| `R` | Toggle auto-reload |
| `Tab` | Switch panel focus |

### Split View
| Key | Action |
|-----|--------|
| `Ctrl+W, v` | Split vertical |
| `Ctrl+W, s` | Split horizontal |
| `Ctrl+W, q` | Close pane |
| `Ctrl+W, w` | Cycle panes |

### Files & URLs
| Key | Action |
|-----|--------|
| `o` | Open file picker |
| `O` | Open URL prompt |
| `H` | View history |
| `m` | View bookmarks |
| `M` | Add bookmark |

### Buffers
| Key | Action |
|-----|--------|
| `B` | Open buffer list |
| `Ctrl+n/p` | Next/prev buffer |
| `Ctrl+x` | Close buffer |

### Other
| Key | Action |
|-----|--------|
| `S` | Settings |
| `?` | Help |
| `q` | Quit |

## GitHub URL Support

Barkdocs can fetch and display markdown from GitHub:

```bash
# Repository root (fetches README)
barkdocs https://github.com/user/repo

# Specific file (blob URL)
barkdocs https://github.com/user/repo/blob/main/docs/guide.md

# Raw URL
barkdocs https://raw.githubusercontent.com/user/repo/main/README.md
```

**Supported branch names:** HEAD, main, master, canary, develop, dev, trunk

Links to GitHub URLs within documents can be followed directly with `Enter` or `f`.

## Configuration

Config file: `~/.config/barkdocs/config.toml`

```toml
theme = "dracula"
line_wrap = true
show_outline = false
show_line_numbers = false
syntax_highlighting = true
auto_reload = true
```

## Data Storage

- History: `~/.local/share/barkdocs/history.json`
- Bookmarks: `~/.local/share/barkdocs/bookmarks.json`

## Themes

11 built-in themes:

| Theme | Description |
|-------|-------------|
| `default` | Clean dark theme |
| `dracula` | Popular dark purple theme |
| `gruvbox` | Retro groove colors |
| `nord` | Arctic, north-bluish palette |
| `solarized-dark` | Precision dark colors |
| `solarized-light` | Precision light colors |
| `monokai` | Sublime-inspired colors |
| `catppuccin` | Soothing pastel theme |
| `tokyo-night` | Dark theme from Tokyo |
| `onedark` | Atom One Dark colors |
| `matrix` | Green on black hacker style |

Change theme with `S` (Settings) or set in config file.

## Environment Variables

| Variable | Description |
|----------|-------------|
| `BARKDOCS_THEME` | Override theme |
| `BARKDOCS_LINE_WRAP` | Override line wrap (1/0) |

## License

MIT OR Apache-2.0
