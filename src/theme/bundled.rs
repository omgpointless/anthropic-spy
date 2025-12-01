// Bundled TOML themes (compiled into binary, extracted on first run)
//
// These themes are written to ~/.config/anthropic-spy/themes/ on first run.
// Users can then modify them freely.

/// Bundled theme: name and TOML content
pub struct BundledTheme {
    pub filename: &'static str,
    pub content: &'static str,
}

/// All bundled themes
pub const BUNDLED_THEMES: &[BundledTheme] = &[
    BundledTheme {
        filename: "One_Half_Dark.toml",
        content: ONE_HALF_DARK,
    },
    BundledTheme {
        filename: "Dracula.toml",
        content: DRACULA,
    },
    BundledTheme {
        filename: "Catppuccin_Mocha.toml",
        content: CATPPUCCIN_MOCHA,
    },
    BundledTheme {
        filename: "Monokai_Pro.toml",
        content: MONOKAI_PRO,
    },
    BundledTheme {
        filename: "Solarized_Light.toml",
        content: SOLARIZED_LIGHT,
    },
    BundledTheme {
        filename: "Nord.toml",
        content: NORD,
    },
    BundledTheme {
        filename: "Gruvbox_Dark.toml",
        content: GRUVBOX_DARK,
    },
    BundledTheme {
        filename: "Monokai_Pro_Ristretto.toml",
        content: MONOKAI_PRO_RISTRETTO,
    },
    BundledTheme {
        filename: "Monokai_Pro_Machine.toml",
        content: MONOKAI_PRO_MACHINE,
    },
    BundledTheme {
        filename: "Monokai_Soda.toml",
        content: MONOKAI_SODA,
    },
    BundledTheme {
        filename: "Tokyo_Night.toml",
        content: TOKYO_NIGHT,
    },
    BundledTheme {
        filename: "GitHub_Dark.toml",
        content: GITHUB_DARK,
    },
    BundledTheme {
        filename: "JetBrains_Darcula.toml",
        content: JETBRAINS_DARCULA,
    },
    BundledTheme {
        filename: "Material_Oceanic.toml",
        content: MATERIAL_OCEANIC,
    },
    BundledTheme {
        filename: "Material_Darker.toml",
        content: MATERIAL_DARKER,
    },
    BundledTheme {
        filename: "Material_Lighter.toml",
        content: MATERIAL_LIGHTER,
    },
    BundledTheme {
        filename: "Material_Palenight.toml",
        content: MATERIAL_PALENIGHT,
    },
    BundledTheme {
        filename: "Material_Deep_Ocean.toml",
        content: MATERIAL_DEEP_OCEAN,
    },
    BundledTheme {
        filename: "Material_Forest.toml",
        content: MATERIAL_FOREST,
    },
    BundledTheme {
        filename: "Material_Sky_Blue.toml",
        content: MATERIAL_SKY_BLUE,
    },
    BundledTheme {
        filename: "Material_Sandy_Beach.toml",
        content: MATERIAL_SANDY_BEACH,
    },
    BundledTheme {
        filename: "Material_Volcano.toml",
        content: MATERIAL_VOLCANO,
    },
    BundledTheme {
        filename: "Material_Space.toml",
        content: MATERIAL_SPACE,
    },
    BundledTheme {
        filename: "Synthwave_84.toml",
        content: SYNTHWAVE_84,
    },
    BundledTheme {
        filename: "Terminal_ANSI.toml",
        content: TERMINAL_ANSI,
    },
    BundledTheme {
        filename: "Rose_Pine.toml",
        content: ROSE_PINE,
    },
    BundledTheme {
        filename: "Everforest_Dark.toml",
        content: EVERFOREST_DARK,
    },
    BundledTheme {
        filename: "Ayu_Mirage.toml",
        content: AYU_MIRAGE,
    },
    BundledTheme {
        filename: "Catppuccin_Latte.toml",
        content: CATPPUCCIN_LATTE,
    },
    BundledTheme {
        filename: "Kanagawa_Wave.toml",
        content: KANAGAWA_WAVE,
    },
];

/// List bundled theme names (for display)
pub fn list_bundled_themes() -> Vec<&'static str> {
    vec![
        "One Half Dark",
        "Dracula",
        "Catppuccin Mocha",
        "Monokai Pro",
        "Solarized Light",
        "Nord",
        "Gruvbox Dark",
        "Monokai Pro Ristretto",
        "Monokai Pro Machine",
        "Monokai Soda",
        "Tokyo Night",
        "GitHub Dark",
        "JetBrains Darcula",
        "Material Oceanic",
        "Material Darker",
        "Material Lighter",
        "Material Palenight",
        "Material Deep Ocean",
        "Material Forest",
        "Material Sky Blue",
        "Material Sandy Beach",
        "Material Volcano",
        "Material Space",
        "Synthwave 84",
        "Terminal ANSI",
        "Rosé Pine",
        "Everforest Dark",
        "Ayu Mirage",
        "Catppuccin Latte",
        "Kanagawa Wave",
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Theme TOML strings
// ─────────────────────────────────────────────────────────────────────────────

pub const ONE_HALF_DARK: &str = r##"# One Half Dark theme for anthropic-spy
# A clean, modern dark theme (default)

[meta]
name = "One Half Dark"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#282c34"
foreground = "#dcdfe4"
border = "#dcdfe4"
border_focused = "#e5c07b"
title = "#56b6c2"
status_bar = "#dcdfe4"
selection_bg = "#474e5d"
selection_fg = "#dcdfe4"

[events]
tool_call = "#56b6c2"
tool_result_ok = "#98c379"
tool_result_fail = "#e06c75"
request = "#61afef"
response = "#c678dd"
error = "#e06c75"
thinking = "#c678dd"
api_usage = "#dcdfe4"
headers = "#dcdfe4"
rate_limit = "#dcdfe4"
context_compact = "#e5c07b"

[context_bar]
fill = "#98c379"
warn = "#e5c07b"
danger = "#e06c75"

[panels]
events = "#56b6c2"
thinking = "#c678dd"
logs = "#98c379"

[vhs]
black = "#282c34"
red = "#e06c75"
green = "#98c379"
yellow = "#e5c07b"
blue = "#61afef"
purple = "#c678dd"
cyan = "#56b6c2"
white = "#dcdfe4"
bright_black = "#5d677a"
bright_red = "#e06c75"
bright_green = "#98c379"
bright_yellow = "#e5c07b"
bright_blue = "#61afef"
bright_purple = "#c678dd"
bright_cyan = "#56b6c2"
bright_white = "#dcdfe4"
cursor = "#a3b3cc"
"##;

pub const DRACULA: &str = r##"# Dracula theme for anthropic-spy
# Popular purple-accented dark theme

[meta]
name = "Dracula"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#282a36"
foreground = "#f8f8f2"
border = "#f8f8f2"
border_focused = "#f1fa8c"
title = "#8be9fd"
status_bar = "#f8f8f2"
selection_bg = "#44475a"
selection_fg = "#f8f8f2"

[events]
tool_call = "#8be9fd"
tool_result_ok = "#50fa7b"
tool_result_fail = "#ff5555"
request = "#bd93f9"
response = "#ff79c6"
error = "#ff5555"
thinking = "#ff79c6"
api_usage = "#f8f8f2"
headers = "#f8f8f2"
rate_limit = "#f8f8f2"
context_compact = "#f1fa8c"

[context_bar]
fill = "#50fa7b"
warn = "#f1fa8c"
danger = "#ff5555"

[panels]
events = "#8be9fd"
thinking = "#ff79c6"
logs = "#50fa7b"

[vhs]
black = "#21222c"
red = "#ff5555"
green = "#50fa7b"
yellow = "#f1fa8c"
blue = "#bd93f9"
purple = "#ff79c6"
cyan = "#8be9fd"
white = "#f8f8f2"
bright_black = "#6272a4"
bright_red = "#ff6e6e"
bright_green = "#69ff94"
bright_yellow = "#ffffa5"
bright_blue = "#d6acff"
bright_purple = "#ff92df"
bright_cyan = "#a4ffff"
bright_white = "#ffffff"
cursor = "#f8f8f2"
"##;

pub const CATPPUCCIN_MOCHA: &str = r##"# Catppuccin Mocha theme for anthropic-spy
# Soothing pastel dark theme

[meta]
name = "Catppuccin Mocha"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#1e1e2e"
foreground = "#cdd6f4"
border = "#585b70"
border_focused = "#f9e2af"
title = "#89b4fa"
status_bar = "#cdd6f4"
selection_bg = "#585b70"
selection_fg = "#cdd6f4"

[events]
tool_call = "#89b4fa"
tool_result_ok = "#a6e3a1"
tool_result_fail = "#f38ba8"
request = "#89b4fa"
response = "#f5c2e7"
error = "#f38ba8"
thinking = "#f5c2e7"
api_usage = "#cdd6f4"
headers = "#cdd6f4"
rate_limit = "#cdd6f4"
context_compact = "#f9e2af"

[context_bar]
fill = "#a6e3a1"
warn = "#f9e2af"
danger = "#f38ba8"

[panels]
events = "#89b4fa"
thinking = "#f5c2e7"
logs = "#a6e3a1"

[vhs]
black = "#45475a"
red = "#f38ba8"
green = "#a6e3a1"
yellow = "#f9e2af"
blue = "#89b4fa"
purple = "#f5c2e7"
cyan = "#94e2d5"
white = "#a6adc8"
bright_black = "#585b70"
bright_red = "#f37799"
bright_green = "#89d88b"
bright_yellow = "#ebd391"
bright_blue = "#74a8fc"
bright_purple = "#f2aede"
bright_cyan = "#6bd7ca"
bright_white = "#bac2de"
cursor = "#f5e0dc"
"##;

pub const MONOKAI_PRO: &str = r##"# Monokai Pro theme for anthropic-spy
# Modern take on the classic Monokai

[meta]
name = "Monokai Pro"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#2d2a2e"
foreground = "#fcfcfa"
border = "#fcfcfa"
border_focused = "#ffd866"
title = "#78dce8"
status_bar = "#fcfcfa"
selection_bg = "#5b595c"
selection_fg = "#fcfcfa"

[events]
tool_call = "#78dce8"
tool_result_ok = "#a9dc76"
tool_result_fail = "#ff6188"
request = "#fc9867"
response = "#ab9df2"
error = "#ff6188"
thinking = "#ab9df2"
api_usage = "#fcfcfa"
headers = "#fcfcfa"
rate_limit = "#fcfcfa"
context_compact = "#ffd866"

[context_bar]
fill = "#a9dc76"
warn = "#ffd866"
danger = "#ff6188"

[panels]
events = "#78dce8"
thinking = "#ab9df2"
logs = "#a9dc76"

[vhs]
black = "#2d2a2e"
red = "#ff6188"
green = "#a9dc76"
yellow = "#ffd866"
blue = "#fc9867"
purple = "#ab9df2"
cyan = "#78dce8"
white = "#fcfcfa"
bright_black = "#727072"
bright_red = "#ff6188"
bright_green = "#a9dc76"
bright_yellow = "#ffd866"
bright_blue = "#fc9867"
bright_purple = "#ab9df2"
bright_cyan = "#78dce8"
bright_white = "#fcfcfa"
cursor = "#c1c0c0"
"##;

pub const SOLARIZED_LIGHT: &str = r##"# Solarized Light theme for anthropic-spy
# Ethan Schoonover's precision color palette (light variant)
#
# NOTE: context_bar.fill uses CYAN (#2aa198) instead of the olive green (#859900)
# because the olive works well for text but looks muddy as a gauge fill.
# This is exactly the kind of fix the new theme format enables!

[meta]
name = "Solarized Light"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#fdf6e3"
foreground = "#657b83"
border = "#93a1a1"
border_focused = "#657b83"
title = "#657b83"
status_bar = "#657b83"
selection_bg = "#eee8d5"
selection_fg = "#657b83"

[events]
tool_call = "#2aa198"
tool_result_ok = "#859900"
tool_result_fail = "#dc322f"
request = "#268bd2"
response = "#d33682"
error = "#dc322f"
thinking = "#d33682"
api_usage = "#657b83"
headers = "#657b83"
rate_limit = "#657b83"
context_compact = "#dc322f"

[context_bar]
# KEY FIX: Using cyan instead of olive green for gauge fill
# The olive (#859900) looks great for text but muddy as a solid fill
fill = "#2aa198"
warn = "#b58900"
danger = "#dc322f"

[panels]
events = "#268bd2"
thinking = "#d33682"
logs = "#859900"

[vhs]
black = "#073642"
red = "#dc322f"
green = "#859900"
yellow = "#b58900"
blue = "#268bd2"
purple = "#d33682"
cyan = "#2aa198"
white = "#bbb5a2"
bright_black = "#002b36"
bright_red = "#cb4b16"
bright_green = "#586e75"
bright_yellow = "#657b83"
bright_blue = "#839496"
bright_purple = "#6c71c4"
bright_cyan = "#93a1a1"
bright_white = "#fdf6e3"
cursor = "#657b83"
"##;

pub const NORD: &str = r##"# Nord theme for anthropic-spy
# Arctic, bluish color palette

[meta]
name = "Nord"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#2e3440"
foreground = "#d8dee9"
border = "#d8dee9"
border_focused = "#ebcb8b"
title = "#88c0d0"
status_bar = "#d8dee9"
selection_bg = "#eceff4"
selection_fg = "#2e3440"

[events]
tool_call = "#88c0d0"
tool_result_ok = "#a3be8c"
tool_result_fail = "#bf616a"
request = "#81a1c1"
response = "#b48ead"
error = "#bf616a"
thinking = "#b48ead"
api_usage = "#d8dee9"
headers = "#d8dee9"
rate_limit = "#d8dee9"
context_compact = "#ebcb8b"

[context_bar]
fill = "#a3be8c"
warn = "#ebcb8b"
danger = "#bf616a"

[panels]
events = "#88c0d0"
thinking = "#b48ead"
logs = "#a3be8c"

[vhs]
black = "#3b4252"
red = "#bf616a"
green = "#a3be8c"
yellow = "#ebcb8b"
blue = "#81a1c1"
purple = "#b48ead"
cyan = "#88c0d0"
white = "#e5e9f0"
bright_black = "#596377"
bright_red = "#bf616a"
bright_green = "#a3be8c"
bright_yellow = "#ebcb8b"
bright_blue = "#81a1c1"
bright_purple = "#b48ead"
bright_cyan = "#8fbcbb"
bright_white = "#eceff4"
cursor = "#eceff4"
"##;

pub const GRUVBOX_DARK: &str = r##"# Gruvbox Dark theme for anthropic-spy
# Retro groove color scheme

[meta]
name = "Gruvbox Dark"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#282828"
foreground = "#ebdbb2"
border = "#ebdbb2"
border_focused = "#fabd2f"
title = "#689d6a"
status_bar = "#ebdbb2"
selection_bg = "#665c54"
selection_fg = "#ebdbb2"

[events]
tool_call = "#689d6a"
tool_result_ok = "#b8bb26"
tool_result_fail = "#fb4934"
request = "#83a598"
response = "#d3869b"
error = "#fb4934"
thinking = "#d3869b"
api_usage = "#ebdbb2"
headers = "#ebdbb2"
rate_limit = "#ebdbb2"
context_compact = "#fabd2f"

[context_bar]
fill = "#b8bb26"
warn = "#fabd2f"
danger = "#fb4934"

[panels]
events = "#689d6a"
thinking = "#d3869b"
logs = "#b8bb26"

[vhs]
black = "#282828"
red = "#cc241d"
green = "#98971a"
yellow = "#d79921"
blue = "#458588"
purple = "#b16286"
cyan = "#689d6a"
white = "#a89984"
bright_black = "#928374"
bright_red = "#fb4934"
bright_green = "#b8bb26"
bright_yellow = "#fabd2f"
bright_blue = "#83a598"
bright_purple = "#d3869b"
bright_cyan = "#8ec07c"
bright_white = "#ebdbb2"
cursor = "#ebdbb2"
"##;

pub const MONOKAI_PRO_RISTRETTO: &str = r##"# Monokai Pro Ristretto theme for anthropic-spy
# Warm, coffee-inspired variant

[meta]
name = "Monokai Pro Ristretto"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#2c2525"
foreground = "#fff1f3"
border = "#fff1f3"
border_focused = "#f9cc6c"
title = "#85dacc"
status_bar = "#fff1f3"
selection_bg = "#5b5353"
selection_fg = "#fff1f3"

[events]
tool_call = "#85dacc"
tool_result_ok = "#adda78"
tool_result_fail = "#fd6883"
request = "#f38d70"
response = "#a8a9eb"
error = "#fd6883"
thinking = "#a8a9eb"
api_usage = "#fff1f3"
headers = "#fff1f3"
rate_limit = "#fff1f3"
context_compact = "#f9cc6c"

[context_bar]
fill = "#adda78"
warn = "#f9cc6c"
danger = "#fd6883"

[panels]
events = "#85dacc"
thinking = "#a8a9eb"
logs = "#adda78"

[vhs]
black = "#2c2525"
red = "#fd6883"
green = "#adda78"
yellow = "#f9cc6c"
blue = "#f38d70"
purple = "#a8a9eb"
cyan = "#85dacc"
white = "#fff1f3"
bright_black = "#72696a"
bright_red = "#fd6883"
bright_green = "#adda78"
bright_yellow = "#f9cc6c"
bright_blue = "#f38d70"
bright_purple = "#a8a9eb"
bright_cyan = "#85dacc"
bright_white = "#fff1f3"
cursor = "#c3b7b8"
"##;

pub const MONOKAI_PRO_MACHINE: &str = r##"# Monokai Pro Machine theme for anthropic-spy
# Industrial, teal-accented variant

[meta]
name = "Monokai Pro Machine"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#273136"
foreground = "#f2fffc"
border = "#f2fffc"
border_focused = "#ffed72"
title = "#7cd5f1"
status_bar = "#f2fffc"
selection_bg = "#545f62"
selection_fg = "#f2fffc"

[events]
tool_call = "#7cd5f1"
tool_result_ok = "#a2e57b"
tool_result_fail = "#ff6d7e"
request = "#ffb270"
response = "#baa0f8"
error = "#ff6d7e"
thinking = "#baa0f8"
api_usage = "#f2fffc"
headers = "#f2fffc"
rate_limit = "#f2fffc"
context_compact = "#ffed72"

[context_bar]
fill = "#a2e57b"
warn = "#ffed72"
danger = "#ff6d7e"

[panels]
events = "#7cd5f1"
thinking = "#baa0f8"
logs = "#a2e57b"

[vhs]
black = "#273136"
red = "#ff6d7e"
green = "#a2e57b"
yellow = "#ffed72"
blue = "#ffb270"
purple = "#baa0f8"
cyan = "#7cd5f1"
white = "#f2fffc"
bright_black = "#6b7678"
bright_red = "#ff6d7e"
bright_green = "#a2e57b"
bright_yellow = "#ffed72"
bright_blue = "#ffb270"
bright_purple = "#baa0f8"
bright_cyan = "#7cd5f1"
bright_white = "#f2fffc"
cursor = "#b8c4c3"
"##;

pub const MONOKAI_SODA: &str = r##"# Monokai Soda theme for anthropic-spy
# Vibrant variant with darker background

[meta]
name = "Monokai Soda"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#1a1a1a"
foreground = "#c4c5b5"
border = "#c4c5b5"
border_focused = "#e0d561"
title = "#58d1eb"
status_bar = "#c4c5b5"
selection_bg = "#343434"
selection_fg = "#f6f6ef"

[events]
tool_call = "#58d1eb"
tool_result_ok = "#98e024"
tool_result_fail = "#f4005f"
request = "#9d65ff"
response = "#f4005f"
error = "#f4005f"
thinking = "#9d65ff"
api_usage = "#c4c5b5"
headers = "#c4c5b5"
rate_limit = "#c4c5b5"
context_compact = "#e0d561"

[context_bar]
fill = "#98e024"
warn = "#fa8419"
danger = "#f4005f"

[panels]
events = "#58d1eb"
thinking = "#9d65ff"
logs = "#98e024"

[vhs]
black = "#1a1a1a"
red = "#f4005f"
green = "#98e024"
yellow = "#fa8419"
blue = "#9d65ff"
purple = "#f4005f"
cyan = "#58d1eb"
white = "#c4c5b5"
bright_black = "#625e4c"
bright_red = "#f4005f"
bright_green = "#98e024"
bright_yellow = "#e0d561"
bright_blue = "#9d65ff"
bright_purple = "#f4005f"
bright_cyan = "#58d1eb"
bright_white = "#f6f6ef"
cursor = "#c4c5b5"
"##;

pub const TOKYO_NIGHT: &str = r##"# Tokyo Night theme for anthropic-spy
# A clean, dark theme inspired by Tokyo at night

[meta]
name = "Tokyo Night"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#1a1b26"
foreground = "#c0caf5"
border = "#c0caf5"
border_focused = "#e0af68"
title = "#7dcfff"
status_bar = "#c0caf5"
selection_bg = "#33467c"
selection_fg = "#c0caf5"

[events]
tool_call = "#7dcfff"
tool_result_ok = "#9ece6a"
tool_result_fail = "#f7768e"
request = "#7aa2f7"
response = "#bb9af7"
error = "#f7768e"
thinking = "#bb9af7"
api_usage = "#c0caf5"
headers = "#c0caf5"
rate_limit = "#c0caf5"
context_compact = "#e0af68"

[context_bar]
fill = "#9ece6a"
warn = "#e0af68"
danger = "#f7768e"

[panels]
events = "#7dcfff"
thinking = "#bb9af7"
logs = "#9ece6a"

[vhs]
black = "#15161e"
red = "#f7768e"
green = "#9ece6a"
yellow = "#e0af68"
blue = "#7aa2f7"
purple = "#bb9af7"
cyan = "#7dcfff"
white = "#a9b1d6"
bright_black = "#414868"
bright_red = "#f7768e"
bright_green = "#9ece6a"
bright_yellow = "#e0af68"
bright_blue = "#7aa2f7"
bright_purple = "#bb9af7"
bright_cyan = "#7dcfff"
bright_white = "#c0caf5"
cursor = "#c0caf5"
"##;

pub const GITHUB_DARK: &str = r##"# GitHub Dark theme for anthropic-spy
# GitHub's official dark theme

[meta]
name = "GitHub Dark"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#101216"
foreground = "#8b949e"
border = "#8b949e"
border_focused = "#e3b341"
title = "#2b7489"
status_bar = "#8b949e"
selection_bg = "#3b5070"
selection_fg = "#ffffff"

[events]
tool_call = "#2b7489"
tool_result_ok = "#56d364"
tool_result_fail = "#f78166"
request = "#6ca4f8"
response = "#db61a2"
error = "#f78166"
thinking = "#db61a2"
api_usage = "#8b949e"
headers = "#8b949e"
rate_limit = "#8b949e"
context_compact = "#e3b341"

[context_bar]
fill = "#56d364"
warn = "#e3b341"
danger = "#f78166"

[panels]
events = "#2b7489"
thinking = "#db61a2"
logs = "#56d364"

[vhs]
black = "#000000"
red = "#f78166"
green = "#56d364"
yellow = "#e3b341"
blue = "#6ca4f8"
purple = "#db61a2"
cyan = "#2b7489"
white = "#ffffff"
bright_black = "#4d4d4d"
bright_red = "#f78166"
bright_green = "#56d364"
bright_yellow = "#e3b341"
bright_blue = "#6ca4f8"
bright_purple = "#db61a2"
bright_cyan = "#2b7489"
bright_white = "#ffffff"
cursor = "#c9d1d9"
"##;

pub const JETBRAINS_DARCULA: &str = r##"# JetBrains Darcula theme for anthropic-spy
# The classic IDE dark theme

[meta]
name = "JetBrains Darcula"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#202020"
foreground = "#adadad"
border = "#adadad"
border_focused = "#ffff00"
title = "#33c2c1"
status_bar = "#adadad"
selection_bg = "#1a3272"
selection_fg = "#eeeeee"

[events]
tool_call = "#33c2c1"
tool_result_ok = "#67ff4f"
tool_result_fail = "#fa5355"
request = "#4581eb"
response = "#fa54ff"
error = "#fa5355"
thinking = "#fa54ff"
api_usage = "#adadad"
headers = "#adadad"
rate_limit = "#adadad"
context_compact = "#ffff00"

[context_bar]
fill = "#126e00"
warn = "#c2c300"
danger = "#fa5355"

[panels]
events = "#33c2c1"
thinking = "#fa54ff"
logs = "#67ff4f"

[vhs]
black = "#000000"
red = "#fa5355"
green = "#126e00"
yellow = "#c2c300"
blue = "#4581eb"
purple = "#fa54ff"
cyan = "#33c2c1"
white = "#adadad"
bright_black = "#555555"
bright_red = "#fb7172"
bright_green = "#67ff4f"
bright_yellow = "#ffff00"
bright_blue = "#6d9df1"
bright_purple = "#fb82ff"
bright_cyan = "#60d3d1"
bright_white = "#eeeeee"
cursor = "#ffffff"
"##;

pub const MATERIAL_OCEANIC: &str = r##"# Material Oceanic theme for anthropic-spy
# Cool blue-green dark theme

[meta]
name = "Material Oceanic"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#263238"
foreground = "#B0BEC5"
border = "#B0BEC5"
border_focused = "#FFCB6B"
title = "#89DDFF"
status_bar = "#B0BEC5"
selection_bg = "#546E7A"
selection_fg = "#EEFFFF"

[events]
tool_call = "#89DDFF"
tool_result_ok = "#C3E88D"
tool_result_fail = "#FF5370"
request = "#82AAFF"
response = "#C792EA"
error = "#FF5370"
thinking = "#C792EA"
api_usage = "#B0BEC5"
headers = "#B0BEC5"
rate_limit = "#B0BEC5"
context_compact = "#FFCB6B"

[context_bar]
fill = "#C3E88D"
warn = "#FFCB6B"
danger = "#FF5370"

[panels]
events = "#89DDFF"
thinking = "#C792EA"
logs = "#C3E88D"

[vhs]
black = "#546E7A"
red = "#FF5370"
green = "#C3E88D"
yellow = "#FFCB6B"
blue = "#82AAFF"
purple = "#C792EA"
cyan = "#89DDFF"
white = "#EEFFFF"
bright_black = "#546E7A"
bright_red = "#F07178"
bright_green = "#C3E88D"
bright_yellow = "#FFCB6B"
bright_blue = "#82AAFF"
bright_purple = "#C792EA"
bright_cyan = "#89DDFF"
bright_white = "#EEFFFF"
cursor = "#FFCB6B"
"##;

pub const MATERIAL_DARKER: &str = r##"# Material Darker theme for anthropic-spy
# Deep charcoal dark theme

[meta]
name = "Material Darker"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#212121"
foreground = "#B0BEC5"
border = "#B0BEC5"
border_focused = "#FFCB6B"
title = "#89DDFF"
status_bar = "#B0BEC5"
selection_bg = "#404040"
selection_fg = "#EEFFFF"

[events]
tool_call = "#89DDFF"
tool_result_ok = "#C3E88D"
tool_result_fail = "#FF5370"
request = "#82AAFF"
response = "#C792EA"
error = "#FF5370"
thinking = "#C792EA"
api_usage = "#B0BEC5"
headers = "#B0BEC5"
rate_limit = "#B0BEC5"
context_compact = "#FFCB6B"

[context_bar]
fill = "#C3E88D"
warn = "#FFCB6B"
danger = "#FF5370"

[panels]
events = "#89DDFF"
thinking = "#C792EA"
logs = "#C3E88D"

[vhs]
black = "#616161"
red = "#FF5370"
green = "#C3E88D"
yellow = "#FFCB6B"
blue = "#82AAFF"
purple = "#C792EA"
cyan = "#89DDFF"
white = "#EEFFFF"
bright_black = "#616161"
bright_red = "#F07178"
bright_green = "#C3E88D"
bright_yellow = "#FFCB6B"
bright_blue = "#82AAFF"
bright_purple = "#C792EA"
bright_cyan = "#89DDFF"
bright_white = "#EEFFFF"
cursor = "#FF9800"
"##;

pub const MATERIAL_LIGHTER: &str = r##"# Material Lighter theme for anthropic-spy
# Clean light theme

[meta]
name = "Material Lighter"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#FAFAFA"
foreground = "#546E7A"
border = "#546E7A"
border_focused = "#F6A434"
title = "#39ADB5"
status_bar = "#546E7A"
selection_bg = "#80CBC4"
selection_fg = "#272727"

[events]
tool_call = "#39ADB5"
tool_result_ok = "#91B859"
tool_result_fail = "#E53935"
request = "#6182B8"
response = "#7C4DFF"
error = "#E53935"
thinking = "#7C4DFF"
api_usage = "#546E7A"
headers = "#546E7A"
rate_limit = "#546E7A"
context_compact = "#F6A434"

[context_bar]
fill = "#91B859"
warn = "#F6A434"
danger = "#E53935"

[panels]
events = "#39ADB5"
thinking = "#7C4DFF"
logs = "#91B859"

[vhs]
black = "#AABFC9"
red = "#E53935"
green = "#91B859"
yellow = "#F6A434"
blue = "#6182B8"
purple = "#7C4DFF"
cyan = "#39ADB5"
white = "#272727"
bright_black = "#AABFC9"
bright_red = "#E53935"
bright_green = "#91B859"
bright_yellow = "#F6A434"
bright_blue = "#6182B8"
bright_purple = "#7C4DFF"
bright_cyan = "#39ADB5"
bright_white = "#272727"
cursor = "#00BCD4"
"##;

pub const MATERIAL_PALENIGHT: &str = r##"# Material Palenight theme for anthropic-spy
# Soft purple-tinted dark theme

[meta]
name = "Material Palenight"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#292D3E"
foreground = "#A6ACCD"
border = "#A6ACCD"
border_focused = "#FFCB6B"
title = "#89DDFF"
status_bar = "#A6ACCD"
selection_bg = "#717CB4"
selection_fg = "#EEFFFF"

[events]
tool_call = "#89DDFF"
tool_result_ok = "#C3E88D"
tool_result_fail = "#FF5370"
request = "#82AAFF"
response = "#C792EA"
error = "#FF5370"
thinking = "#C792EA"
api_usage = "#A6ACCD"
headers = "#A6ACCD"
rate_limit = "#A6ACCD"
context_compact = "#FFCB6B"

[context_bar]
fill = "#C3E88D"
warn = "#FFCB6B"
danger = "#FF5370"

[panels]
events = "#89DDFF"
thinking = "#C792EA"
logs = "#C3E88D"

[vhs]
black = "#676E95"
red = "#FF5370"
green = "#C3E88D"
yellow = "#FFCB6B"
blue = "#82AAFF"
purple = "#C792EA"
cyan = "#89DDFF"
white = "#EEFFFF"
bright_black = "#676E95"
bright_red = "#F07178"
bright_green = "#C3E88D"
bright_yellow = "#FFCB6B"
bright_blue = "#82AAFF"
bright_purple = "#C792EA"
bright_cyan = "#89DDFF"
bright_white = "#EEFFFF"
cursor = "#AB47BC"
"##;

pub const MATERIAL_DEEP_OCEAN: &str = r##"# Material Deep Ocean theme for anthropic-spy
# Ultra-dark blue theme

[meta]
name = "Material Deep Ocean"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#0F111A"
foreground = "#8F93A2"
border = "#8F93A2"
border_focused = "#FFCB6B"
title = "#89DDFF"
status_bar = "#8F93A2"
selection_bg = "#717CB4"
selection_fg = "#EEFFFF"

[events]
tool_call = "#89DDFF"
tool_result_ok = "#C3E88D"
tool_result_fail = "#FF5370"
request = "#82AAFF"
response = "#C792EA"
error = "#FF5370"
thinking = "#C792EA"
api_usage = "#8F93A2"
headers = "#8F93A2"
rate_limit = "#8F93A2"
context_compact = "#FFCB6B"

[context_bar]
fill = "#C3E88D"
warn = "#FFCB6B"
danger = "#FF5370"

[panels]
events = "#89DDFF"
thinking = "#C792EA"
logs = "#C3E88D"

[vhs]
black = "#717CB4"
red = "#FF5370"
green = "#C3E88D"
yellow = "#FFCB6B"
blue = "#82AAFF"
purple = "#C792EA"
cyan = "#89DDFF"
white = "#EEFFFF"
bright_black = "#717CB4"
bright_red = "#F07178"
bright_green = "#C3E88D"
bright_yellow = "#FFCB6B"
bright_blue = "#82AAFF"
bright_purple = "#C792EA"
bright_cyan = "#89DDFF"
bright_white = "#EEFFFF"
cursor = "#84FFFF"
"##;

pub const MATERIAL_FOREST: &str = r##"# Material Forest theme for anthropic-spy
# Deep green nature theme

[meta]
name = "Material Forest"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#002626"
foreground = "#B2C2B0"
border = "#B2C2B0"
border_focused = "#FFCB6B"
title = "#89DDFF"
status_bar = "#B2C2B0"
selection_bg = "#1E611E"
selection_fg = "#EEFFFF"

[events]
tool_call = "#89DDFF"
tool_result_ok = "#C3E88D"
tool_result_fail = "#FF5370"
request = "#82AAFF"
response = "#C792EA"
error = "#FF5370"
thinking = "#C792EA"
api_usage = "#B2C2B0"
headers = "#B2C2B0"
rate_limit = "#B2C2B0"
context_compact = "#FFCB6B"

[context_bar]
fill = "#C3E88D"
warn = "#FFCB6B"
danger = "#FF5370"

[panels]
events = "#89DDFF"
thinking = "#C792EA"
logs = "#C3E88D"

[vhs]
black = "#005454"
red = "#FF5370"
green = "#C3E88D"
yellow = "#FFCB6B"
blue = "#82AAFF"
purple = "#C792EA"
cyan = "#89DDFF"
white = "#EEFFFF"
bright_black = "#005454"
bright_red = "#F07178"
bright_green = "#C3E88D"
bright_yellow = "#FFCB6B"
bright_blue = "#82AAFF"
bright_purple = "#C792EA"
bright_cyan = "#89DDFF"
bright_white = "#EEFFFF"
cursor = "#FFCC80"
"##;

pub const MATERIAL_SKY_BLUE: &str = r##"# Material Sky Blue theme for anthropic-spy
# Bright light theme with blue accents

[meta]
name = "Material Sky Blue"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#F5F5F5"
foreground = "#005761"
border = "#005761"
border_focused = "#F6A434"
title = "#39ADB5"
status_bar = "#005761"
selection_bg = "#ADE2EB"
selection_fg = "#272727"

[events]
tool_call = "#39ADB5"
tool_result_ok = "#91B859"
tool_result_fail = "#E53935"
request = "#6182B8"
response = "#7C4DFF"
error = "#E53935"
thinking = "#7C4DFF"
api_usage = "#005761"
headers = "#005761"
rate_limit = "#005761"
context_compact = "#F6A434"

[context_bar]
fill = "#91B859"
warn = "#F6A434"
danger = "#E53935"

[panels]
events = "#39ADB5"
thinking = "#7C4DFF"
logs = "#91B859"

[vhs]
black = "#01579B"
red = "#E53935"
green = "#91B859"
yellow = "#F6A434"
blue = "#6182B8"
purple = "#7C4DFF"
cyan = "#39ADB5"
white = "#272727"
bright_black = "#01579B"
bright_red = "#E53935"
bright_green = "#91B859"
bright_yellow = "#F6A434"
bright_blue = "#6182B8"
bright_purple = "#7C4DFF"
bright_cyan = "#39ADB5"
bright_white = "#272727"
cursor = "#00C6E0"
"##;

pub const MATERIAL_SANDY_BEACH: &str = r##"# Material Sandy Beach theme for anthropic-spy
# Warm cream light theme

[meta]
name = "Material Sandy Beach"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#FFF8ED"
foreground = "#546E7A"
border = "#546E7A"
border_focused = "#F6A434"
title = "#39ADB5"
status_bar = "#546E7A"
selection_bg = "#E7C496"
selection_fg = "#272727"

[events]
tool_call = "#39ADB5"
tool_result_ok = "#91B859"
tool_result_fail = "#E53935"
request = "#6182B8"
response = "#7C4DFF"
error = "#E53935"
thinking = "#7C4DFF"
api_usage = "#546E7A"
headers = "#546E7A"
rate_limit = "#546E7A"
context_compact = "#F6A434"

[context_bar]
fill = "#39ADB5"
warn = "#F6A434"
danger = "#E53935"

[panels]
events = "#39ADB5"
thinking = "#7C4DFF"
logs = "#91B859"

[vhs]
black = "#888477"
red = "#E53935"
green = "#91B859"
yellow = "#F6A434"
blue = "#6182B8"
purple = "#7C4DFF"
cyan = "#39ADB5"
white = "#272727"
bright_black = "#888477"
bright_red = "#E53935"
bright_green = "#91B859"
bright_yellow = "#F6A434"
bright_blue = "#6182B8"
bright_purple = "#7C4DFF"
bright_cyan = "#39ADB5"
bright_white = "#272727"
cursor = "#53C7F0"
"##;

pub const MATERIAL_VOLCANO: &str = r##"# Material Volcano theme for anthropic-spy
# Deep red dark theme

[meta]
name = "Material Volcano"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#390000"
foreground = "#FFEAEA"
border = "#FFEAEA"
border_focused = "#FFCB6B"
title = "#89DDFF"
status_bar = "#FFEAEA"
selection_bg = "#750000"
selection_fg = "#EEFFFF"

[events]
tool_call = "#89DDFF"
tool_result_ok = "#C3E88D"
tool_result_fail = "#FF5370"
request = "#82AAFF"
response = "#C792EA"
error = "#FF5370"
thinking = "#C792EA"
api_usage = "#FFEAEA"
headers = "#FFEAEA"
rate_limit = "#FFEAEA"
context_compact = "#FFCB6B"

[context_bar]
fill = "#C3E88D"
warn = "#FFCB6B"
danger = "#FF5370"

[panels]
events = "#89DDFF"
thinking = "#C792EA"
logs = "#C3E88D"

[vhs]
black = "#7F6451"
red = "#FF5370"
green = "#C3E88D"
yellow = "#FFCB6B"
blue = "#82AAFF"
purple = "#C792EA"
cyan = "#89DDFF"
white = "#EEFFFF"
bright_black = "#7F6451"
bright_red = "#F07178"
bright_green = "#C3E88D"
bright_yellow = "#FFCB6B"
bright_blue = "#82AAFF"
bright_purple = "#C792EA"
bright_cyan = "#89DDFF"
bright_white = "#EEFFFF"
cursor = "#00BCD4"
"##;

pub const MATERIAL_SPACE: &str = r##"# Material Space theme for anthropic-spy
# Deep blue space theme

[meta]
name = "Material Space"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#1B2240"
foreground = "#EFEFF1"
border = "#EFEFF1"
border_focused = "#FFCB6B"
title = "#89DDFF"
status_bar = "#EFEFF1"
selection_bg = "#383F56"
selection_fg = "#EEFFFF"

[events]
tool_call = "#89DDFF"
tool_result_ok = "#C3E88D"
tool_result_fail = "#FF5370"
request = "#82AAFF"
response = "#C792EA"
error = "#FF5370"
thinking = "#C792EA"
api_usage = "#EFEFF1"
headers = "#EFEFF1"
rate_limit = "#EFEFF1"
context_compact = "#FFCB6B"

[context_bar]
fill = "#C3E88D"
warn = "#FFCB6B"
danger = "#FF5370"

[panels]
events = "#89DDFF"
thinking = "#C792EA"
logs = "#C3E88D"

[vhs]
black = "#959DAA"
red = "#FF5370"
green = "#C3E88D"
yellow = "#FFCB6B"
blue = "#82AAFF"
purple = "#C792EA"
cyan = "#89DDFF"
white = "#EEFFFF"
bright_black = "#959DAA"
bright_red = "#F07178"
bright_green = "#C3E88D"
bright_yellow = "#FFCB6B"
bright_blue = "#82AAFF"
bright_purple = "#C792EA"
bright_cyan = "#89DDFF"
bright_white = "#EEFFFF"
cursor = "#AD9BF6"
"##;

pub const SYNTHWAVE_84: &str = r##"# Synthwave 84 theme for anthropic-spy
# Retro neon synthwave theme

[meta]
name = "Synthwave 84"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#2A2139"
foreground = "#FFFFFF"
border = "#FFFFFF"
border_focused = "#FEDE5D"
title = "#36F9F6"
status_bar = "#FFFFFF"
selection_bg = "#463465"
selection_fg = "#FFFFFF"

[events]
tool_call = "#36F9F6"
tool_result_ok = "#72F1B8"
tool_result_fail = "#FE4450"
request = "#34D3FB"
response = "#FF7EDB"
error = "#FE4450"
thinking = "#FF7EDB"
api_usage = "#FFFFFF"
headers = "#FFFFFF"
rate_limit = "#FFFFFF"
context_compact = "#FEDE5D"

[context_bar]
fill = "#72F1B8"
warn = "#FEDE5D"
danger = "#FE4450"

[panels]
events = "#36F9F6"
thinking = "#FF7EDB"
logs = "#72F1B8"

[vhs]
black = "#848BBD"
red = "#FE4450"
green = "#72F1B8"
yellow = "#FEDE5D"
blue = "#34D3FB"
purple = "#FF7EDB"
cyan = "#36F9F6"
white = "#B6B1B1"
bright_black = "#848BBD"
bright_red = "#FE4450"
bright_green = "#72F1B8"
bright_yellow = "#FEDE5D"
bright_blue = "#34D3FB"
bright_purple = "#FF7EDB"
bright_cyan = "#36F9F6"
bright_white = "#FFFFFF"
cursor = "#F92AAD"
"##;

pub const TERMINAL_ANSI: &str = r##"# Terminal ANSI theme for anthropic-spy
# Uses your terminal's native ANSI colors - adapts to your terminal theme!
#
# This theme uses "ansi:X" syntax instead of hex colors:
# - ansi:0-7 = standard colors (black, red, green, yellow, blue, magenta, cyan, white)
# - ansi:8-15 = bright variants
# - ansi:fg = terminal's default foreground
# - ansi:bg = terminal's default background (transparent)
#
# Perfect for users who have carefully crafted their terminal theme and want
# anthropic-spy to inherit those colors automatically.

[meta]
name = "Terminal ANSI"
version = 1
author = "anthropic-spy"

[ui]
background = "ansi:bg"
foreground = "ansi:fg"
border = "ansi:fg"
border_focused = "ansi:3"
title = "ansi:6"
status_bar = "ansi:fg"
selection_bg = "ansi:8"
selection_fg = "ansi:fg"

[events]
tool_call = "ansi:6"
tool_result_ok = "ansi:2"
tool_result_fail = "ansi:1"
request = "ansi:4"
response = "ansi:5"
error = "ansi:1"
thinking = "ansi:5"
api_usage = "ansi:fg"
headers = "ansi:fg"
rate_limit = "ansi:fg"
context_compact = "ansi:3"

[context_bar]
fill = "ansi:2"
warn = "ansi:3"
danger = "ansi:1"

[panels]
events = "ansi:6"
thinking = "ansi:5"
logs = "ansi:2"

[vhs]
black = "ansi:0"
red = "ansi:1"
green = "ansi:2"
yellow = "ansi:3"
blue = "ansi:4"
purple = "ansi:5"
cyan = "ansi:6"
white = "ansi:7"
bright_black = "ansi:8"
bright_red = "ansi:9"
bright_green = "ansi:10"
bright_yellow = "ansi:11"
bright_blue = "ansi:12"
bright_purple = "ansi:13"
bright_cyan = "ansi:14"
bright_white = "ansi:15"
cursor = "ansi:fg"
"##;

pub const ROSE_PINE: &str = r##"# Rosé Pine theme for anthropic-spy
# All natural pine, faux fur and a bit of soho vibes

[meta]
name = "Rosé Pine"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#191724"
foreground = "#e0def4"
border = "#e0def4"
border_focused = "#f6c177"
title = "#ebbcba"
status_bar = "#e0def4"
selection_bg = "#403d52"
selection_fg = "#e0def4"

[events]
tool_call = "#9ccfd8"
tool_result_ok = "#31748f"
tool_result_fail = "#eb6f92"
request = "#9ccfd8"
response = "#c4a7e7"
error = "#eb6f92"
thinking = "#c4a7e7"
api_usage = "#e0def4"
headers = "#e0def4"
rate_limit = "#e0def4"
context_compact = "#f6c177"

[context_bar]
fill = "#31748f"
warn = "#f6c177"
danger = "#eb6f92"

[panels]
events = "#9ccfd8"
thinking = "#c4a7e7"
logs = "#31748f"

[vhs]
black = "#26233a"
red = "#eb6f92"
green = "#31748f"
yellow = "#f6c177"
blue = "#9ccfd8"
purple = "#c4a7e7"
cyan = "#ebbcba"
white = "#e0def4"
bright_black = "#6e6a86"
bright_red = "#eb6f92"
bright_green = "#31748f"
bright_yellow = "#f6c177"
bright_blue = "#9ccfd8"
bright_purple = "#c4a7e7"
bright_cyan = "#ebbcba"
bright_white = "#e0def4"
cursor = "#e0def4"
"##;

pub const EVERFOREST_DARK: &str = r##"# Everforest Dark theme for anthropic-spy
# Green nature-inspired dark theme, easy on the eyes

[meta]
name = "Everforest Dark"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#1e2326"
foreground = "#d3c6aa"
border = "#d3c6aa"
border_focused = "#dbbc7f"
title = "#83c092"
status_bar = "#d3c6aa"
selection_bg = "#4c3743"
selection_fg = "#fffbef"

[events]
tool_call = "#83c092"
tool_result_ok = "#a7c080"
tool_result_fail = "#e67e80"
request = "#7fbbb3"
response = "#d699b6"
error = "#e67e80"
thinking = "#d699b6"
api_usage = "#d3c6aa"
headers = "#d3c6aa"
rate_limit = "#d3c6aa"
context_compact = "#dbbc7f"

[context_bar]
fill = "#a7c080"
warn = "#dbbc7f"
danger = "#e67e80"

[panels]
events = "#83c092"
thinking = "#d699b6"
logs = "#a7c080"

[vhs]
black = "#7a8478"
red = "#e67e80"
green = "#a7c080"
yellow = "#dbbc7f"
blue = "#7fbbb3"
purple = "#d699b6"
cyan = "#83c092"
white = "#f2efdf"
bright_black = "#a6b0a0"
bright_red = "#f85552"
bright_green = "#8da101"
bright_yellow = "#dfa000"
bright_blue = "#3a94c5"
bright_purple = "#df69ba"
bright_cyan = "#35a77c"
bright_white = "#fffbef"
cursor = "#e69875"
"##;

pub const AYU_MIRAGE: &str = r##"# Ayu Mirage theme for anthropic-spy
# Modern dark theme with soft colors

[meta]
name = "Ayu Mirage"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#1f2430"
foreground = "#cccac2"
border = "#cccac2"
border_focused = "#facc6e"
title = "#90e1c6"
status_bar = "#cccac2"
selection_bg = "#409fff"
selection_fg = "#ffffff"

[events]
tool_call = "#90e1c6"
tool_result_ok = "#87d96c"
tool_result_fail = "#ed8274"
request = "#6dcbfa"
response = "#dabafa"
error = "#ed8274"
thinking = "#dabafa"
api_usage = "#cccac2"
headers = "#cccac2"
rate_limit = "#cccac2"
context_compact = "#facc6e"

[context_bar]
fill = "#87d96c"
warn = "#facc6e"
danger = "#ed8274"

[panels]
events = "#6dcbfa"
thinking = "#dabafa"
logs = "#87d96c"

[vhs]
black = "#171b24"
red = "#ed8274"
green = "#87d96c"
yellow = "#facc6e"
blue = "#6dcbfa"
purple = "#dabafa"
cyan = "#90e1c6"
white = "#c7c7c7"
bright_black = "#686868"
bright_red = "#f28779"
bright_green = "#d5ff80"
bright_yellow = "#ffd173"
bright_blue = "#73d0ff"
bright_purple = "#dfbfff"
bright_cyan = "#95e6cb"
bright_white = "#ffffff"
cursor = "#ffcc66"
"##;

pub const CATPPUCCIN_LATTE: &str = r##"# Catppuccin Latte theme for anthropic-spy
# Soothing pastel light theme

[meta]
name = "Catppuccin Latte"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#eff1f5"
foreground = "#4c4f69"
border = "#4c4f69"
border_focused = "#df8e1d"
title = "#179299"
status_bar = "#4c4f69"
selection_bg = "#acb0be"
selection_fg = "#4c4f69"

[events]
tool_call = "#179299"
tool_result_ok = "#40a02b"
tool_result_fail = "#d20f39"
request = "#1e66f5"
response = "#ea76cb"
error = "#d20f39"
thinking = "#ea76cb"
api_usage = "#4c4f69"
headers = "#4c4f69"
rate_limit = "#4c4f69"
context_compact = "#df8e1d"

[context_bar]
fill = "#40a02b"
warn = "#df8e1d"
danger = "#d20f39"

[panels]
events = "#1e66f5"
thinking = "#ea76cb"
logs = "#40a02b"

[vhs]
black = "#5c5f77"
red = "#d20f39"
green = "#40a02b"
yellow = "#df8e1d"
blue = "#1e66f5"
purple = "#ea76cb"
cyan = "#179299"
white = "#acb0be"
bright_black = "#6c6f85"
bright_red = "#de293e"
bright_green = "#49af3d"
bright_yellow = "#eea02d"
bright_blue = "#456eff"
bright_purple = "#fe85d8"
bright_cyan = "#2d9fa8"
bright_white = "#bcc0cc"
cursor = "#dc8a78"
"##;

pub const KANAGAWA_WAVE: &str = r##"# Kanagawa Wave theme for anthropic-spy
# Inspired by Katsushika Hokusai's famous painting

[meta]
name = "Kanagawa Wave"
version = 1
author = "iTerm2-Color-Schemes"

[ui]
background = "#1f1f28"
foreground = "#dcd7ba"
border = "#dcd7ba"
border_focused = "#c0a36e"
title = "#6a9589"
status_bar = "#dcd7ba"
selection_bg = "#2d4f67"
selection_fg = "#dcd7ba"

[events]
tool_call = "#6a9589"
tool_result_ok = "#76946a"
tool_result_fail = "#c34043"
request = "#7e9cd8"
response = "#957fb8"
error = "#c34043"
thinking = "#957fb8"
api_usage = "#dcd7ba"
headers = "#dcd7ba"
rate_limit = "#dcd7ba"
context_compact = "#c0a36e"

[context_bar]
fill = "#76946a"
warn = "#c0a36e"
danger = "#c34043"

[panels]
events = "#7e9cd8"
thinking = "#957fb8"
logs = "#98bb6c"

[vhs]
black = "#090618"
red = "#c34043"
green = "#76946a"
yellow = "#c0a36e"
blue = "#7e9cd8"
purple = "#957fb8"
cyan = "#6a9589"
white = "#c8c093"
bright_black = "#727169"
bright_red = "#e82424"
bright_green = "#98bb6c"
bright_yellow = "#e6c384"
bright_blue = "#7fb4ca"
bright_purple = "#938aa9"
bright_cyan = "#7aa89f"
bright_white = "#dcd7ba"
cursor = "#c8c093"
"##;
