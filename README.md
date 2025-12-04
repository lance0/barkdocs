# Barkdocs

A keyboard-driven TUI markdown viewer, companion to [barklog](https://github.com/lance0/barklog).

## Features

- **Fast markdown rendering** - View markdown files with syntax highlighting
- **Vim-like navigation** - Use familiar j/k keys to scroll
- **Search** - Find text with `/` and navigate with n/N
- **Outline panel** - Quick navigation via document headings
- **Split view** - View two sections side-by-side
- **11 themes** - Match your terminal aesthetic

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
# View a specific file
barkdocs README.md

# Auto-open README.md in current directory
barkdocs
```

## Keybindings

| Key | Action |
|-----|--------|
| j/k | Scroll down/up |
| g/G | Go to top/bottom |
| / | Search |
| n/N | Next/prev match |
| b | Toggle outline |
| w | Toggle line wrap |
| ? | Help |
| q | Quit |

## Configuration

Config file: `~/.config/barkdocs/config.toml`

```toml
theme = "dracula"
line_wrap = true
show_outline = true
```

## Themes

Available themes:
- default
- dracula
- gruvbox
- nord
- solarized-dark
- solarized-light
- monokai
- catppuccin
- tokyo-night
- onedark
- matrix

## License

MIT OR Apache-2.0
