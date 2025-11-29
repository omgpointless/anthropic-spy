// Embedded theme palettes (compiled into binary)
//
// Popular themes from iTerm2-Color-Schemes/vhs are included here
// for zero-config usage. Users can add more themes via external JSON files.
//
// Source: https://github.com/mbadolato/iTerm2-Color-Schemes

/// Get JSON for an embedded theme by name
pub fn get_embedded_theme(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase().replace([' ', '-', '_'], "");
    match name_lower.as_str() {
        "onehalfdark" | "basic" => Some(ONE_HALF_DARK),
        "dracula" => Some(DRACULA),
        "nord" => Some(NORD),
        "gruvbox" | "gruvboxdark" => Some(GRUVBOX_DARK),
        "monokai" | "monokaipro" => Some(MONOKAI_PRO),
        "monokaiproristretto" | "ristretto" => Some(MONOKAI_PRO_RISTRETTO),
        "monokaipromachine" | "machine" => Some(MONOKAI_PRO_MACHINE),
        "monokaisoda" | "soda" => Some(MONOKAI_SODA),
        "tokyonight" => Some(TOKYO_NIGHT),
        "catppuccin" | "catppuccinmocha" => Some(CATPPUCCIN_MOCHA),
        "github" | "githubdark" => Some(GITHUB_DARK),
        "jetbrains" | "jetbrainsdarcula" | "darcula" => Some(JETBRAINS_DARCULA),
        "terminal" => Some(TERMINAL_ANSI),
        _ => None,
    }
}

/// List all embedded theme names (display names)
pub fn list_embedded_themes() -> &'static [&'static str] {
    &[
        "One Half Dark",
        "Dracula",
        "Nord",
        "Gruvbox Dark",
        "Monokai Pro",
        "Monokai Pro Ristretto",
        "Monokai Pro Machine",
        "Monokai Soda",
        "TokyoNight",
        "Catppuccin Mocha",
        "GitHub Dark",
        "JetBrains Darcula",
        "Terminal",
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Embedded theme JSON strings (using r##"..."## to allow # in hex colors)
// ─────────────────────────────────────────────────────────────────────────────

/// One Half Dark - Clean, modern theme (our default "basic" theme)
pub const ONE_HALF_DARK: &str = r##"{
  "name": "One Half Dark",
  "black": "#282c34",
  "red": "#e06c75",
  "green": "#98c379",
  "yellow": "#e5c07b",
  "blue": "#61afef",
  "purple": "#c678dd",
  "cyan": "#56b6c2",
  "white": "#dcdfe4",
  "brightBlack": "#5d677a",
  "brightRed": "#e06c75",
  "brightGreen": "#98c379",
  "brightYellow": "#e5c07b",
  "brightBlue": "#61afef",
  "brightPurple": "#c678dd",
  "brightCyan": "#56b6c2",
  "brightWhite": "#dcdfe4",
  "background": "#282c34",
  "foreground": "#dcdfe4",
  "cursor": "#a3b3cc",
  "selection": "#474e5d"
}"##;

/// Dracula - Popular purple-accented dark theme
pub const DRACULA: &str = r##"{
  "name": "Dracula",
  "black": "#21222c",
  "red": "#ff5555",
  "green": "#50fa7b",
  "yellow": "#f1fa8c",
  "blue": "#bd93f9",
  "purple": "#ff79c6",
  "cyan": "#8be9fd",
  "white": "#f8f8f2",
  "brightBlack": "#6272a4",
  "brightRed": "#ff6e6e",
  "brightGreen": "#69ff94",
  "brightYellow": "#ffffa5",
  "brightBlue": "#d6acff",
  "brightPurple": "#ff92df",
  "brightCyan": "#a4ffff",
  "brightWhite": "#ffffff",
  "background": "#282a36",
  "foreground": "#f8f8f2",
  "cursor": "#f8f8f2",
  "selection": "#44475a"
}"##;

/// Nord - Arctic, bluish color palette
pub const NORD: &str = r##"{
  "name": "Nord",
  "black": "#3b4252",
  "red": "#bf616a",
  "green": "#a3be8c",
  "yellow": "#ebcb8b",
  "blue": "#81a1c1",
  "purple": "#b48ead",
  "cyan": "#88c0d0",
  "white": "#e5e9f0",
  "brightBlack": "#596377",
  "brightRed": "#bf616a",
  "brightGreen": "#a3be8c",
  "brightYellow": "#ebcb8b",
  "brightBlue": "#81a1c1",
  "brightPurple": "#b48ead",
  "brightCyan": "#8fbcbb",
  "brightWhite": "#eceff4",
  "background": "#2e3440",
  "foreground": "#d8dee9",
  "cursor": "#eceff4",
  "selection": "#eceff4"
}"##;

/// Gruvbox Dark - Retro groove color scheme
pub const GRUVBOX_DARK: &str = r##"{
  "name": "Gruvbox Dark",
  "black": "#282828",
  "red": "#cc241d",
  "green": "#98971a",
  "yellow": "#d79921",
  "blue": "#458588",
  "purple": "#b16286",
  "cyan": "#689d6a",
  "white": "#a89984",
  "brightBlack": "#928374",
  "brightRed": "#fb4934",
  "brightGreen": "#b8bb26",
  "brightYellow": "#fabd2f",
  "brightBlue": "#83a598",
  "brightPurple": "#d3869b",
  "brightCyan": "#8ec07c",
  "brightWhite": "#ebdbb2",
  "background": "#282828",
  "foreground": "#ebdbb2",
  "cursor": "#ebdbb2",
  "selection": "#665c54"
}"##;

/// Monokai Pro - Modern take on the classic Monokai
pub const MONOKAI_PRO: &str = r##"{
  "name": "Monokai Pro",
  "black": "#2d2a2e",
  "red": "#ff6188",
  "green": "#a9dc76",
  "yellow": "#ffd866",
  "blue": "#fc9867",
  "purple": "#ab9df2",
  "cyan": "#78dce8",
  "white": "#fcfcfa",
  "brightBlack": "#727072",
  "brightRed": "#ff6188",
  "brightGreen": "#a9dc76",
  "brightYellow": "#ffd866",
  "brightBlue": "#fc9867",
  "brightPurple": "#ab9df2",
  "brightCyan": "#78dce8",
  "brightWhite": "#fcfcfa",
  "background": "#2d2a2e",
  "foreground": "#fcfcfa",
  "cursor": "#c1c0c0",
  "selection": "#5b595c"
}"##;

/// Monokai Pro Ristretto - Warm, coffee-inspired variant
pub const MONOKAI_PRO_RISTRETTO: &str = r##"{
  "name": "Monokai Pro Ristretto",
  "black": "#2c2525",
  "red": "#fd6883",
  "green": "#adda78",
  "yellow": "#f9cc6c",
  "blue": "#f38d70",
  "purple": "#a8a9eb",
  "cyan": "#85dacc",
  "white": "#fff1f3",
  "brightBlack": "#72696a",
  "brightRed": "#fd6883",
  "brightGreen": "#adda78",
  "brightYellow": "#f9cc6c",
  "brightBlue": "#f38d70",
  "brightPurple": "#a8a9eb",
  "brightCyan": "#85dacc",
  "brightWhite": "#fff1f3",
  "background": "#2c2525",
  "foreground": "#fff1f3",
  "cursor": "#c3b7b8",
  "selection": "#5b5353"
}"##;

/// Monokai Pro Machine - Industrial, teal-accented variant
pub const MONOKAI_PRO_MACHINE: &str = r##"{
  "name": "Monokai Pro Machine",
  "black": "#273136",
  "red": "#ff6d7e",
  "green": "#a2e57b",
  "yellow": "#ffed72",
  "blue": "#ffb270",
  "purple": "#baa0f8",
  "cyan": "#7cd5f1",
  "white": "#f2fffc",
  "brightBlack": "#6b7678",
  "brightRed": "#ff6d7e",
  "brightGreen": "#a2e57b",
  "brightYellow": "#ffed72",
  "brightBlue": "#ffb270",
  "brightPurple": "#baa0f8",
  "brightCyan": "#7cd5f1",
  "brightWhite": "#f2fffc",
  "background": "#273136",
  "foreground": "#f2fffc",
  "cursor": "#b8c4c3",
  "selection": "#545f62"
}"##;

/// Monokai Soda - Vibrant variant with darker background (from Gogh)
pub const MONOKAI_SODA: &str = r##"{
  "name": "Monokai Soda",
  "black": "#1a1a1a",
  "red": "#f4005f",
  "green": "#98e024",
  "yellow": "#fa8419",
  "blue": "#9d65ff",
  "purple": "#f4005f",
  "cyan": "#58d1eb",
  "white": "#c4c5b5",
  "brightBlack": "#625e4c",
  "brightRed": "#f4005f",
  "brightGreen": "#98e024",
  "brightYellow": "#e0d561",
  "brightBlue": "#9d65ff",
  "brightPurple": "#f4005f",
  "brightCyan": "#58d1eb",
  "brightWhite": "#f6f6ef",
  "background": "#1a1a1a",
  "foreground": "#c4c5b5",
  "cursor": "#c4c5b5",
  "selection": "#343434"
}"##;

/// TokyoNight - A clean, dark theme inspired by Tokyo at night
pub const TOKYO_NIGHT: &str = r##"{
  "name": "TokyoNight",
  "black": "#15161e",
  "red": "#f7768e",
  "green": "#9ece6a",
  "yellow": "#e0af68",
  "blue": "#7aa2f7",
  "purple": "#bb9af7",
  "cyan": "#7dcfff",
  "white": "#a9b1d6",
  "brightBlack": "#414868",
  "brightRed": "#f7768e",
  "brightGreen": "#9ece6a",
  "brightYellow": "#e0af68",
  "brightBlue": "#7aa2f7",
  "brightPurple": "#bb9af7",
  "brightCyan": "#7dcfff",
  "brightWhite": "#c0caf5",
  "background": "#1a1b26",
  "foreground": "#c0caf5",
  "cursor": "#c0caf5",
  "selection": "#33467c"
}"##;

/// Catppuccin Mocha - Soothing pastel theme
pub const CATPPUCCIN_MOCHA: &str = r##"{
  "name": "Catppuccin Mocha",
  "black": "#45475a",
  "red": "#f38ba8",
  "green": "#a6e3a1",
  "yellow": "#f9e2af",
  "blue": "#89b4fa",
  "purple": "#f5c2e7",
  "cyan": "#94e2d5",
  "white": "#a6adc8",
  "brightBlack": "#585b70",
  "brightRed": "#f37799",
  "brightGreen": "#89d88b",
  "brightYellow": "#ebd391",
  "brightBlue": "#74a8fc",
  "brightPurple": "#f2aede",
  "brightCyan": "#6bd7ca",
  "brightWhite": "#bac2de",
  "background": "#1e1e2e",
  "foreground": "#cdd6f4",
  "cursor": "#f5e0dc",
  "selection": "#585b70"
}"##;

/// GitHub Dark - GitHub's official dark theme
pub const GITHUB_DARK: &str = r##"{
  "name": "GitHub Dark",
  "black": "#000000",
  "red": "#f78166",
  "green": "#56d364",
  "yellow": "#e3b341",
  "blue": "#6ca4f8",
  "purple": "#db61a2",
  "cyan": "#2b7489",
  "white": "#ffffff",
  "brightBlack": "#4d4d4d",
  "brightRed": "#f78166",
  "brightGreen": "#56d364",
  "brightYellow": "#e3b341",
  "brightBlue": "#6ca4f8",
  "brightPurple": "#db61a2",
  "brightCyan": "#2b7489",
  "brightWhite": "#ffffff",
  "background": "#101216",
  "foreground": "#8b949e",
  "cursor": "#c9d1d9",
  "selection": "#3b5070"
}"##;

/// JetBrains Darcula - The classic IDE dark theme
pub const JETBRAINS_DARCULA: &str = r##"{
  "name": "JetBrains Darcula",
  "black": "#000000",
  "red": "#fa5355",
  "green": "#126e00",
  "yellow": "#c2c300",
  "blue": "#4581eb",
  "purple": "#fa54ff",
  "cyan": "#33c2c1",
  "white": "#adadad",
  "brightBlack": "#555555",
  "brightRed": "#fb7172",
  "brightGreen": "#67ff4f",
  "brightYellow": "#ffff00",
  "brightBlue": "#6d9df1",
  "brightPurple": "#fb82ff",
  "brightCyan": "#60d3d1",
  "brightWhite": "#eeeeee",
  "background": "#202020",
  "foreground": "#adadad",
  "cursor": "#ffffff",
  "selection": "#1a3272"
}"##;

/// Terminal ANSI - Uses terminal's native ANSI colors (adapts to your terminal theme)
/// This is a special palette that maps to ANSI color codes instead of RGB
pub const TERMINAL_ANSI: &str = r##"{
  "name": "Terminal",
  "black": "ansi:0",
  "red": "ansi:1",
  "green": "ansi:2",
  "yellow": "ansi:3",
  "blue": "ansi:4",
  "purple": "ansi:5",
  "cyan": "ansi:6",
  "white": "ansi:7",
  "brightBlack": "ansi:8",
  "brightRed": "ansi:9",
  "brightGreen": "ansi:10",
  "brightYellow": "ansi:11",
  "brightBlue": "ansi:12",
  "brightPurple": "ansi:13",
  "brightCyan": "ansi:14",
  "brightWhite": "ansi:15",
  "background": "ansi:bg",
  "foreground": "ansi:fg",
  "cursor": "ansi:fg",
  "selection": "ansi:8"
}"##;
