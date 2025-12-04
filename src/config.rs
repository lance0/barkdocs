use crate::theme::Theme;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for barkdocs
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Theme name
    pub theme: String,
    /// Whether to show line wrapping by default
    pub line_wrap: bool,
    /// Whether to show the outline panel by default
    pub show_outline: bool,
    /// Outline panel width
    pub outline_width: u16,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            line_wrap: true,
            show_outline: true,
            outline_width: 24,
            show_line_numbers: false,
        }
    }
}

impl Config {
    /// Get the config file path
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("barkdocs").join("config.toml"))
    }

    /// Load config from file, applying environment variable overrides
    pub fn load() -> Self {
        let mut config = Self::load_from_file().unwrap_or_default();
        config.apply_env_overrides();
        config
    }

    /// Load config from file only
    fn load_from_file() -> Option<Self> {
        let path = Self::config_path()?;
        let contents = std::fs::read_to_string(&path).ok()?;
        toml::from_str(&contents).ok()
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&mut self) {
        if let Ok(theme) = std::env::var("BARKDOCS_THEME") {
            self.theme = theme;
        }
        // Also support BARK_THEME for ecosystem consistency
        if let Ok(theme) = std::env::var("BARK_THEME") {
            self.theme = theme;
        }

        if let Ok(wrap) = std::env::var("BARKDOCS_LINE_WRAP") {
            self.line_wrap = matches!(wrap.to_lowercase().as_str(), "1" | "true" | "yes");
        }

        if let Ok(outline) = std::env::var("BARKDOCS_OUTLINE") {
            self.show_outline = matches!(outline.to_lowercase().as_str(), "1" | "true" | "yes");
        }

        if let Ok(line_numbers) = std::env::var("BARKDOCS_LINE_NUMBERS") {
            self.show_line_numbers =
                matches!(line_numbers.to_lowercase().as_str(), "1" | "true" | "yes");
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Could not determine config path")?;

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let contents =
            toml::to_string_pretty(self).map_err(|e| format!("Failed to serialize config: {}", e))?;

        std::fs::write(&path, contents).map_err(|e| format!("Failed to write config: {}", e))
    }

    /// Get the theme based on config
    pub fn get_theme(&self) -> Theme {
        Theme::by_name(&self.theme)
    }
}
