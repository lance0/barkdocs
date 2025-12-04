use ratatui::style::Color;

/// Theme definition for barkdocs UI
#[derive(Clone, Debug)]
pub struct Theme {
    pub name: &'static str,

    // UI borders
    pub border_focused: Color,
    pub border_unfocused: Color,

    // Header
    pub header_title: Color,
    pub header_filename: Color,
    pub header_bg: Color,

    // Status bar
    pub status_mode_bg: Color,
    pub status_mode_fg: Color,
    pub status_help: Color,
    pub status_bg: Color,

    // Search highlights
    pub highlight_match_bg: Color,
    pub highlight_match_fg: Color,

    // Markdown elements
    pub heading_1: Color,
    pub heading_2: Color,
    pub heading_3: Color,
    pub heading_other: Color,
    pub code_block_bg: Color,
    pub code_inline: Color,
    pub link: Color,
    #[allow(dead_code)]
    pub emphasis: Color,
    pub strong: Color,
    pub blockquote: Color,
    pub list_marker: Color,
    pub horizontal_rule: Color,

    // Outline panel
    pub outline_selected: Color,
    pub outline_heading: Color,
    #[allow(dead_code)]
    pub outline_current: Color,

    // General text
    pub text: Color,
    pub text_muted: Color,

    // Empty states / messages
    pub empty_state: Color,
    pub warning_message: Color,

    // Help overlay
    pub help_border: Color,
    pub help_bg: Color,
}

impl Theme {
    /// Get theme by name
    pub fn by_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dracula" => Self::dracula(),
            "gruvbox" => Self::gruvbox(),
            "nord" => Self::nord(),
            "solarized" | "solarized-dark" => Self::solarized_dark(),
            "solarized-light" => Self::solarized_light(),
            "monokai" => Self::monokai(),
            "catppuccin" => Self::catppuccin(),
            "tokyo-night" | "tokyonight" => Self::tokyo_night(),
            "onedark" | "one-dark" => Self::onedark(),
            "matrix" => Self::matrix(),
            _ => Self::default_theme(),
        }
    }

    /// List all available theme names
    pub fn available_themes() -> &'static [&'static str] {
        &[
            "default",
            "dracula",
            "gruvbox",
            "nord",
            "solarized-dark",
            "solarized-light",
            "monokai",
            "catppuccin",
            "tokyo-night",
            "onedark",
            "matrix",
        ]
    }

    /// Default theme - modern dark with vibrant accents
    pub fn default_theme() -> Self {
        Self {
            name: "default",

            border_focused: Color::Rgb(99, 179, 237), // soft blue
            border_unfocused: Color::Rgb(74, 85, 104), // muted slate

            header_title: Color::Rgb(129, 230, 217),   // teal
            header_filename: Color::Rgb(99, 179, 237), // soft blue
            header_bg: Color::Rgb(26, 32, 44),         // dark navy

            status_mode_bg: Color::Rgb(129, 230, 217), // teal
            status_mode_fg: Color::Rgb(26, 32, 44),    // dark navy
            status_help: Color::Rgb(113, 128, 150),    // slate gray
            status_bg: Color::Rgb(26, 32, 44),         // dark navy

            highlight_match_bg: Color::Rgb(250, 204, 21), // golden yellow
            highlight_match_fg: Color::Rgb(26, 32, 44),   // dark navy

            heading_1: Color::Rgb(246, 135, 179),     // pink
            heading_2: Color::Rgb(129, 230, 217),     // teal
            heading_3: Color::Rgb(250, 204, 21),      // golden
            heading_other: Color::Rgb(183, 148, 244), // lavender
            code_block_bg: Color::Rgb(45, 55, 72),    // slate dark
            code_inline: Color::Rgb(245, 158, 11),    // amber
            link: Color::Rgb(99, 179, 237),           // soft blue
            emphasis: Color::Rgb(246, 135, 179),      // pink
            strong: Color::Rgb(247, 250, 252),        // bright white
            blockquote: Color::Rgb(113, 128, 150),    // slate gray
            list_marker: Color::Rgb(129, 230, 217),   // teal
            horizontal_rule: Color::Rgb(74, 85, 104), // muted slate

            outline_selected: Color::Rgb(250, 204, 21), // golden
            outline_heading: Color::Rgb(226, 232, 240), // light gray
            outline_current: Color::Rgb(129, 230, 217), // teal

            text: Color::Rgb(226, 232, 240),       // light gray
            text_muted: Color::Rgb(113, 128, 150), // slate gray

            empty_state: Color::Rgb(113, 128, 150), // slate gray
            warning_message: Color::Rgb(250, 204, 21), // golden

            help_border: Color::Rgb(183, 148, 244), // lavender
            help_bg: Color::Rgb(26, 32, 44),        // dark navy
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        Self {
            name: "dracula",

            border_focused: Color::Rgb(189, 147, 249), // purple
            border_unfocused: Color::Rgb(68, 71, 90),  // comment

            header_title: Color::Rgb(80, 250, 123), // green
            header_filename: Color::Rgb(139, 233, 253), // cyan
            header_bg: Color::Rgb(40, 42, 54),      // background

            status_mode_bg: Color::Rgb(189, 147, 249),
            status_mode_fg: Color::Rgb(40, 42, 54),
            status_help: Color::Rgb(98, 114, 164),
            status_bg: Color::Rgb(40, 42, 54),

            highlight_match_bg: Color::Rgb(255, 184, 108), // orange
            highlight_match_fg: Color::Rgb(40, 42, 54),

            heading_1: Color::Rgb(255, 121, 198),    // pink
            heading_2: Color::Rgb(189, 147, 249),    // purple
            heading_3: Color::Rgb(139, 233, 253),    // cyan
            heading_other: Color::Rgb(80, 250, 123), // green
            code_block_bg: Color::Rgb(68, 71, 90),
            code_inline: Color::Rgb(255, 184, 108),
            link: Color::Rgb(139, 233, 253),
            emphasis: Color::Rgb(255, 121, 198),
            strong: Color::Rgb(248, 248, 242),
            blockquote: Color::Rgb(98, 114, 164),
            list_marker: Color::Rgb(255, 121, 198),
            horizontal_rule: Color::Rgb(68, 71, 90),

            outline_selected: Color::Rgb(255, 184, 108),
            outline_heading: Color::Rgb(248, 248, 242),
            outline_current: Color::Rgb(189, 147, 249),

            text: Color::Rgb(248, 248, 242),
            text_muted: Color::Rgb(98, 114, 164),

            empty_state: Color::Rgb(98, 114, 164),
            warning_message: Color::Rgb(255, 184, 108),

            help_border: Color::Rgb(255, 121, 198),
            help_bg: Color::Rgb(40, 42, 54),
        }
    }

    /// Gruvbox theme
    pub fn gruvbox() -> Self {
        Self {
            name: "gruvbox",

            border_focused: Color::Rgb(215, 153, 33), // yellow
            border_unfocused: Color::Rgb(102, 92, 84), // gray

            header_title: Color::Rgb(184, 187, 38), // green
            header_filename: Color::Rgb(131, 165, 152), // aqua
            header_bg: Color::Rgb(40, 40, 40),      // bg0

            status_mode_bg: Color::Rgb(215, 153, 33),
            status_mode_fg: Color::Rgb(40, 40, 40),
            status_help: Color::Rgb(146, 131, 116),
            status_bg: Color::Rgb(40, 40, 40),

            highlight_match_bg: Color::Rgb(254, 128, 25), // orange
            highlight_match_fg: Color::Rgb(40, 40, 40),

            heading_1: Color::Rgb(251, 73, 52),       // red
            heading_2: Color::Rgb(215, 153, 33),      // yellow
            heading_3: Color::Rgb(184, 187, 38),      // green
            heading_other: Color::Rgb(131, 165, 152), // aqua
            code_block_bg: Color::Rgb(60, 56, 54),
            code_inline: Color::Rgb(254, 128, 25),
            link: Color::Rgb(131, 165, 152),
            emphasis: Color::Rgb(211, 134, 155), // purple
            strong: Color::Rgb(235, 219, 178),
            blockquote: Color::Rgb(146, 131, 116),
            list_marker: Color::Rgb(215, 153, 33),
            horizontal_rule: Color::Rgb(102, 92, 84),

            outline_selected: Color::Rgb(254, 128, 25),
            outline_heading: Color::Rgb(235, 219, 178),
            outline_current: Color::Rgb(215, 153, 33),

            text: Color::Rgb(235, 219, 178),
            text_muted: Color::Rgb(146, 131, 116),

            empty_state: Color::Rgb(146, 131, 116),
            warning_message: Color::Rgb(254, 128, 25),

            help_border: Color::Rgb(211, 134, 155),
            help_bg: Color::Rgb(40, 40, 40),
        }
    }

    /// Nord theme
    pub fn nord() -> Self {
        Self {
            name: "nord",

            border_focused: Color::Rgb(136, 192, 208), // frost
            border_unfocused: Color::Rgb(76, 86, 106), // polar night

            header_title: Color::Rgb(163, 190, 140), // green
            header_filename: Color::Rgb(136, 192, 208), // frost
            header_bg: Color::Rgb(46, 52, 64),       // polar night

            status_mode_bg: Color::Rgb(136, 192, 208),
            status_mode_fg: Color::Rgb(46, 52, 64),
            status_help: Color::Rgb(76, 86, 106),
            status_bg: Color::Rgb(46, 52, 64),

            highlight_match_bg: Color::Rgb(208, 135, 112), // orange
            highlight_match_fg: Color::Rgb(46, 52, 64),

            heading_1: Color::Rgb(191, 97, 106),      // red
            heading_2: Color::Rgb(208, 135, 112),     // orange
            heading_3: Color::Rgb(235, 203, 139),     // yellow
            heading_other: Color::Rgb(163, 190, 140), // green
            code_block_bg: Color::Rgb(59, 66, 82),
            code_inline: Color::Rgb(208, 135, 112),
            link: Color::Rgb(129, 161, 193),
            emphasis: Color::Rgb(180, 142, 173), // purple
            strong: Color::Rgb(236, 239, 244),
            blockquote: Color::Rgb(76, 86, 106),
            list_marker: Color::Rgb(136, 192, 208),
            horizontal_rule: Color::Rgb(76, 86, 106),

            outline_selected: Color::Rgb(235, 203, 139),
            outline_heading: Color::Rgb(236, 239, 244),
            outline_current: Color::Rgb(136, 192, 208),

            text: Color::Rgb(236, 239, 244),
            text_muted: Color::Rgb(76, 86, 106),

            empty_state: Color::Rgb(76, 86, 106),
            warning_message: Color::Rgb(235, 203, 139),

            help_border: Color::Rgb(180, 142, 173),
            help_bg: Color::Rgb(46, 52, 64),
        }
    }

    /// Solarized Dark theme
    pub fn solarized_dark() -> Self {
        Self {
            name: "solarized-dark",

            border_focused: Color::Rgb(38, 139, 210), // blue
            border_unfocused: Color::Rgb(88, 110, 117), // base01

            header_title: Color::Rgb(133, 153, 0),     // green
            header_filename: Color::Rgb(42, 161, 152), // cyan
            header_bg: Color::Rgb(0, 43, 54),          // base03

            status_mode_bg: Color::Rgb(38, 139, 210),
            status_mode_fg: Color::Rgb(0, 43, 54),
            status_help: Color::Rgb(88, 110, 117),
            status_bg: Color::Rgb(0, 43, 54),

            highlight_match_bg: Color::Rgb(181, 137, 0), // yellow
            highlight_match_fg: Color::Rgb(0, 43, 54),

            heading_1: Color::Rgb(220, 50, 47),     // red
            heading_2: Color::Rgb(203, 75, 22),     // orange
            heading_3: Color::Rgb(181, 137, 0),     // yellow
            heading_other: Color::Rgb(133, 153, 0), // green
            code_block_bg: Color::Rgb(7, 54, 66),
            code_inline: Color::Rgb(203, 75, 22),
            link: Color::Rgb(38, 139, 210),
            emphasis: Color::Rgb(108, 113, 196), // violet
            strong: Color::Rgb(147, 161, 161),
            blockquote: Color::Rgb(88, 110, 117),
            list_marker: Color::Rgb(42, 161, 152),
            horizontal_rule: Color::Rgb(88, 110, 117),

            outline_selected: Color::Rgb(181, 137, 0),
            outline_heading: Color::Rgb(147, 161, 161),
            outline_current: Color::Rgb(38, 139, 210),

            text: Color::Rgb(147, 161, 161),
            text_muted: Color::Rgb(88, 110, 117),

            empty_state: Color::Rgb(88, 110, 117),
            warning_message: Color::Rgb(181, 137, 0),

            help_border: Color::Rgb(108, 113, 196),
            help_bg: Color::Rgb(0, 43, 54),
        }
    }

    /// Solarized Light theme
    pub fn solarized_light() -> Self {
        Self {
            name: "solarized-light",

            border_focused: Color::Rgb(38, 139, 210),
            border_unfocused: Color::Rgb(147, 161, 161),

            header_title: Color::Rgb(133, 153, 0),
            header_filename: Color::Rgb(42, 161, 152),
            header_bg: Color::Rgb(238, 232, 213),

            status_mode_bg: Color::Rgb(38, 139, 210),
            status_mode_fg: Color::Rgb(253, 246, 227),
            status_help: Color::Rgb(147, 161, 161),
            status_bg: Color::Rgb(238, 232, 213),

            highlight_match_bg: Color::Rgb(181, 137, 0),
            highlight_match_fg: Color::Rgb(253, 246, 227),

            heading_1: Color::Rgb(220, 50, 47),
            heading_2: Color::Rgb(203, 75, 22),
            heading_3: Color::Rgb(181, 137, 0),
            heading_other: Color::Rgb(133, 153, 0),
            code_block_bg: Color::Rgb(253, 246, 227),
            code_inline: Color::Rgb(203, 75, 22),
            link: Color::Rgb(38, 139, 210),
            emphasis: Color::Rgb(108, 113, 196),
            strong: Color::Rgb(88, 110, 117),
            blockquote: Color::Rgb(147, 161, 161),
            list_marker: Color::Rgb(42, 161, 152),
            horizontal_rule: Color::Rgb(147, 161, 161),

            outline_selected: Color::Rgb(181, 137, 0),
            outline_heading: Color::Rgb(88, 110, 117),
            outline_current: Color::Rgb(38, 139, 210),

            text: Color::Rgb(88, 110, 117),
            text_muted: Color::Rgb(147, 161, 161),

            empty_state: Color::Rgb(147, 161, 161),
            warning_message: Color::Rgb(181, 137, 0),

            help_border: Color::Rgb(108, 113, 196),
            help_bg: Color::Rgb(238, 232, 213),
        }
    }

    /// Monokai theme
    pub fn monokai() -> Self {
        Self {
            name: "monokai",

            border_focused: Color::Rgb(249, 38, 114), // pink
            border_unfocused: Color::Rgb(117, 113, 94),

            header_title: Color::Rgb(166, 226, 46), // green
            header_filename: Color::Rgb(102, 217, 239), // cyan
            header_bg: Color::Rgb(39, 40, 34),

            status_mode_bg: Color::Rgb(249, 38, 114),
            status_mode_fg: Color::Rgb(39, 40, 34),
            status_help: Color::Rgb(117, 113, 94),
            status_bg: Color::Rgb(39, 40, 34),

            highlight_match_bg: Color::Rgb(253, 151, 31), // orange
            highlight_match_fg: Color::Rgb(39, 40, 34),

            heading_1: Color::Rgb(249, 38, 114),
            heading_2: Color::Rgb(253, 151, 31),
            heading_3: Color::Rgb(230, 219, 116), // yellow
            heading_other: Color::Rgb(166, 226, 46),
            code_block_bg: Color::Rgb(52, 53, 46),
            code_inline: Color::Rgb(253, 151, 31),
            link: Color::Rgb(102, 217, 239),
            emphasis: Color::Rgb(174, 129, 255), // purple
            strong: Color::Rgb(248, 248, 242),
            blockquote: Color::Rgb(117, 113, 94),
            list_marker: Color::Rgb(249, 38, 114),
            horizontal_rule: Color::Rgb(117, 113, 94),

            outline_selected: Color::Rgb(253, 151, 31),
            outline_heading: Color::Rgb(248, 248, 242),
            outline_current: Color::Rgb(249, 38, 114),

            text: Color::Rgb(248, 248, 242),
            text_muted: Color::Rgb(117, 113, 94),

            empty_state: Color::Rgb(117, 113, 94),
            warning_message: Color::Rgb(253, 151, 31),

            help_border: Color::Rgb(174, 129, 255),
            help_bg: Color::Rgb(39, 40, 34),
        }
    }

    /// Catppuccin (Mocha) theme
    pub fn catppuccin() -> Self {
        Self {
            name: "catppuccin",

            border_focused: Color::Rgb(203, 166, 247), // mauve
            border_unfocused: Color::Rgb(88, 91, 112), // surface2

            header_title: Color::Rgb(166, 227, 161), // green
            header_filename: Color::Rgb(137, 220, 235), // sky
            header_bg: Color::Rgb(30, 30, 46),       // base

            status_mode_bg: Color::Rgb(203, 166, 247),
            status_mode_fg: Color::Rgb(30, 30, 46),
            status_help: Color::Rgb(108, 112, 134),
            status_bg: Color::Rgb(30, 30, 46),

            highlight_match_bg: Color::Rgb(250, 179, 135), // peach
            highlight_match_fg: Color::Rgb(30, 30, 46),

            heading_1: Color::Rgb(243, 139, 168),     // pink
            heading_2: Color::Rgb(203, 166, 247),     // mauve
            heading_3: Color::Rgb(249, 226, 175),     // yellow
            heading_other: Color::Rgb(166, 227, 161), // green
            code_block_bg: Color::Rgb(49, 50, 68),
            code_inline: Color::Rgb(250, 179, 135),
            link: Color::Rgb(137, 180, 250),     // blue
            emphasis: Color::Rgb(245, 194, 231), // pink lighter
            strong: Color::Rgb(205, 214, 244),
            blockquote: Color::Rgb(108, 112, 134),
            list_marker: Color::Rgb(243, 139, 168),
            horizontal_rule: Color::Rgb(88, 91, 112),

            outline_selected: Color::Rgb(250, 179, 135),
            outline_heading: Color::Rgb(205, 214, 244),
            outline_current: Color::Rgb(203, 166, 247),

            text: Color::Rgb(205, 214, 244),
            text_muted: Color::Rgb(108, 112, 134),

            empty_state: Color::Rgb(108, 112, 134),
            warning_message: Color::Rgb(250, 179, 135),

            help_border: Color::Rgb(243, 139, 168),
            help_bg: Color::Rgb(30, 30, 46),
        }
    }

    /// Tokyo Night theme
    pub fn tokyo_night() -> Self {
        Self {
            name: "tokyo-night",

            border_focused: Color::Rgb(122, 162, 247), // blue
            border_unfocused: Color::Rgb(65, 72, 104), // comment

            header_title: Color::Rgb(158, 206, 106), // green
            header_filename: Color::Rgb(125, 207, 255), // cyan
            header_bg: Color::Rgb(26, 27, 38),       // bg

            status_mode_bg: Color::Rgb(122, 162, 247),
            status_mode_fg: Color::Rgb(26, 27, 38),
            status_help: Color::Rgb(86, 95, 137),
            status_bg: Color::Rgb(26, 27, 38),

            highlight_match_bg: Color::Rgb(255, 158, 100), // orange
            highlight_match_fg: Color::Rgb(26, 27, 38),

            heading_1: Color::Rgb(247, 118, 142),     // red
            heading_2: Color::Rgb(187, 154, 247),     // purple
            heading_3: Color::Rgb(224, 175, 104),     // yellow
            heading_other: Color::Rgb(158, 206, 106), // green
            code_block_bg: Color::Rgb(41, 46, 66),
            code_inline: Color::Rgb(255, 158, 100),
            link: Color::Rgb(122, 162, 247),
            emphasis: Color::Rgb(187, 154, 247),
            strong: Color::Rgb(192, 202, 245),
            blockquote: Color::Rgb(86, 95, 137),
            list_marker: Color::Rgb(125, 207, 255),
            horizontal_rule: Color::Rgb(65, 72, 104),

            outline_selected: Color::Rgb(255, 158, 100),
            outline_heading: Color::Rgb(192, 202, 245),
            outline_current: Color::Rgb(122, 162, 247),

            text: Color::Rgb(192, 202, 245),
            text_muted: Color::Rgb(86, 95, 137),

            empty_state: Color::Rgb(86, 95, 137),
            warning_message: Color::Rgb(224, 175, 104),

            help_border: Color::Rgb(187, 154, 247),
            help_bg: Color::Rgb(26, 27, 38),
        }
    }

    /// One Dark theme
    pub fn onedark() -> Self {
        Self {
            name: "onedark",

            border_focused: Color::Rgb(97, 175, 239), // blue
            border_unfocused: Color::Rgb(92, 99, 112), // comment

            header_title: Color::Rgb(152, 195, 121), // green
            header_filename: Color::Rgb(86, 182, 194), // cyan
            header_bg: Color::Rgb(40, 44, 52),       // bg

            status_mode_bg: Color::Rgb(97, 175, 239),
            status_mode_fg: Color::Rgb(40, 44, 52),
            status_help: Color::Rgb(92, 99, 112),
            status_bg: Color::Rgb(40, 44, 52),

            highlight_match_bg: Color::Rgb(209, 154, 102), // orange
            highlight_match_fg: Color::Rgb(40, 44, 52),

            heading_1: Color::Rgb(224, 108, 117),     // red
            heading_2: Color::Rgb(198, 120, 221),     // purple
            heading_3: Color::Rgb(229, 192, 123),     // yellow
            heading_other: Color::Rgb(152, 195, 121), // green
            code_block_bg: Color::Rgb(50, 55, 65),
            code_inline: Color::Rgb(209, 154, 102),
            link: Color::Rgb(97, 175, 239),
            emphasis: Color::Rgb(198, 120, 221),
            strong: Color::Rgb(171, 178, 191),
            blockquote: Color::Rgb(92, 99, 112),
            list_marker: Color::Rgb(86, 182, 194),
            horizontal_rule: Color::Rgb(92, 99, 112),

            outline_selected: Color::Rgb(209, 154, 102),
            outline_heading: Color::Rgb(171, 178, 191),
            outline_current: Color::Rgb(97, 175, 239),

            text: Color::Rgb(171, 178, 191),
            text_muted: Color::Rgb(92, 99, 112),

            empty_state: Color::Rgb(92, 99, 112),
            warning_message: Color::Rgb(229, 192, 123),

            help_border: Color::Rgb(198, 120, 221),
            help_bg: Color::Rgb(40, 44, 52),
        }
    }

    /// Matrix theme (for fun)
    pub fn matrix() -> Self {
        Self {
            name: "matrix",

            border_focused: Color::Rgb(0, 255, 0),
            border_unfocused: Color::Rgb(0, 80, 0),

            header_title: Color::Rgb(0, 255, 0),
            header_filename: Color::Rgb(0, 200, 0),
            header_bg: Color::Black,

            status_mode_bg: Color::Rgb(0, 255, 0),
            status_mode_fg: Color::Black,
            status_help: Color::Rgb(0, 100, 0),
            status_bg: Color::Black,

            highlight_match_bg: Color::Rgb(0, 255, 0),
            highlight_match_fg: Color::Black,

            heading_1: Color::Rgb(0, 255, 0),
            heading_2: Color::Rgb(0, 220, 0),
            heading_3: Color::Rgb(0, 180, 0),
            heading_other: Color::Rgb(0, 150, 0),
            code_block_bg: Color::Rgb(0, 20, 0),
            code_inline: Color::Rgb(100, 255, 100),
            link: Color::Rgb(0, 200, 100),
            emphasis: Color::Rgb(0, 180, 0),
            strong: Color::Rgb(0, 255, 0),
            blockquote: Color::Rgb(0, 100, 0),
            list_marker: Color::Rgb(0, 255, 0),
            horizontal_rule: Color::Rgb(0, 80, 0),

            outline_selected: Color::Rgb(0, 255, 0),
            outline_heading: Color::Rgb(0, 200, 0),
            outline_current: Color::Rgb(0, 255, 0),

            text: Color::Rgb(0, 200, 0),
            text_muted: Color::Rgb(0, 100, 0),

            empty_state: Color::Rgb(0, 80, 0),
            warning_message: Color::Rgb(150, 255, 0),

            help_border: Color::Rgb(0, 255, 0),
            help_bg: Color::Black,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_theme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert_eq!(theme.name, "default");
    }

    #[test]
    fn test_by_name_default() {
        let theme = Theme::by_name("default");
        assert_eq!(theme.name, "default");
    }

    #[test]
    fn test_by_name_case_insensitive() {
        let theme1 = Theme::by_name("DRACULA");
        let theme2 = Theme::by_name("dracula");
        let theme3 = Theme::by_name("Dracula");

        assert_eq!(theme1.name, "dracula");
        assert_eq!(theme2.name, "dracula");
        assert_eq!(theme3.name, "dracula");
    }

    #[test]
    fn test_by_name_unknown_returns_default() {
        let theme = Theme::by_name("nonexistent-theme");
        assert_eq!(theme.name, "default");
    }

    #[test]
    fn test_all_themes_loadable() {
        for name in Theme::available_themes() {
            let theme = Theme::by_name(name);
            // Just verify it doesn't panic and has valid name
            assert!(!theme.name.is_empty());
        }
    }

    #[test]
    fn test_available_themes_includes_known() {
        let themes = Theme::available_themes();
        assert!(themes.contains(&"default"));
        assert!(themes.contains(&"dracula"));
        assert!(themes.contains(&"gruvbox"));
        assert!(themes.contains(&"nord"));
        assert!(themes.contains(&"catppuccin"));
    }

    #[test]
    fn test_theme_aliases() {
        // Test solarized alias
        let theme1 = Theme::by_name("solarized");
        let theme2 = Theme::by_name("solarized-dark");
        assert_eq!(theme1.name, theme2.name);

        // Test tokyo-night aliases
        let theme3 = Theme::by_name("tokyo-night");
        let theme4 = Theme::by_name("tokyonight");
        assert_eq!(theme3.name, theme4.name);

        // Test onedark aliases
        let theme5 = Theme::by_name("onedark");
        let theme6 = Theme::by_name("one-dark");
        assert_eq!(theme5.name, theme6.name);
    }

    #[test]
    fn test_dracula_theme() {
        let theme = Theme::dracula();
        assert_eq!(theme.name, "dracula");
        // Verify it's actually dracula colors (purple border)
        assert_eq!(theme.border_focused, Color::Rgb(189, 147, 249));
    }

    #[test]
    fn test_gruvbox_theme() {
        let theme = Theme::gruvbox();
        assert_eq!(theme.name, "gruvbox");
    }

    #[test]
    fn test_nord_theme() {
        let theme = Theme::nord();
        assert_eq!(theme.name, "nord");
    }

    #[test]
    fn test_matrix_theme() {
        let theme = Theme::matrix();
        assert_eq!(theme.name, "matrix");
        // Matrix should be all green
        assert_eq!(theme.border_focused, Color::Rgb(0, 255, 0));
    }

    #[test]
    fn test_solarized_light_vs_dark() {
        let dark = Theme::solarized_dark();
        let light = Theme::solarized_light();

        // They should have different names
        assert_eq!(dark.name, "solarized-dark");
        assert_eq!(light.name, "solarized-light");

        // And different background colors
        assert_ne!(dark.header_bg, light.header_bg);
    }
}
