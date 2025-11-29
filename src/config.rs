// Configuration for the proxy server
//
// Configuration is loaded in order of precedence:
// 1. Environment variables (highest priority)
// 2. Config file (~/.config/anthropic-spy/config.toml)
// 3. Built-in defaults (lowest priority)

use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;

/// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Feature flags for optional modules (opt-out: default enabled)
#[derive(Debug, Clone)]
pub struct Features {
    /// Storage module: write events to JSONL files
    pub storage: bool,

    /// Thinking panel: show Claude's extended thinking
    pub thinking_panel: bool,

    /// Stats tracking: token counts, costs, tool distribution
    pub stats: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            storage: true,
            thinking_panel: true,
            stats: true,
        }
    }
}

/// Augmentation settings
///
/// Augmentations modify API responses by injecting additional content.
/// Context warning is enabled by default as it's non-intrusive and helpful.
#[derive(Debug, Clone)]
pub struct Augmentation {
    /// Context warning: inject usage alerts when context fills up
    /// Adds styled annotations suggesting /compact when thresholds are crossed
    pub context_warning: bool,

    /// Thresholds at which to warn (percentages)
    /// Default: [60, 80, 85, 90, 95]
    pub context_warning_thresholds: Vec<u8>,
}

impl Default for Augmentation {
    fn default() -> Self {
        Self {
            context_warning: true, // Enabled by default
            context_warning_thresholds: vec![60, 80, 85, 90, 95],
        }
    }
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Address to bind the proxy server to
    pub bind_addr: SocketAddr,

    /// Target Anthropic API URL
    pub api_url: String,

    /// Directory for storing logs
    pub log_dir: PathBuf,

    /// Whether to enable the TUI (can be disabled for headless mode)
    pub enable_tui: bool,

    /// Demo mode: generate mock events for showcasing the TUI
    pub demo_mode: bool,

    /// Context window limit for the gauge (empirically ~147K triggers compact)
    pub context_limit: u64,

    /// Theme name: "basic", "terminal", "dracula", "monokai", "nord", "gruvbox"
    pub theme: String,

    /// Use theme's background color (true) or terminal's default (false)
    pub use_theme_background: bool,

    /// Feature flags for optional modules
    pub features: Features,

    /// Augmentation settings (opt-in response modifications)
    pub augmentation: Augmentation,
}

/// Feature flags as loaded from config file
#[derive(Debug, Deserialize, Default)]
struct FileFeatures {
    storage: Option<bool>,
    thinking_panel: Option<bool>,
    stats: Option<bool>,
}

/// Augmentation settings as loaded from config file
#[derive(Debug, Deserialize, Default)]
struct FileAugmentation {
    context_warning: Option<bool>,
    context_warning_thresholds: Option<Vec<u8>>,
}

/// Config file structure (subset of Config that makes sense to persist)
#[derive(Debug, Deserialize, Default)]
struct FileConfig {
    context_limit: Option<u64>,
    bind_addr: Option<String>,
    api_url: Option<String>,
    log_dir: Option<String>,
    theme: Option<String>,
    use_theme_background: Option<bool>,

    /// Optional [features] section
    features: Option<FileFeatures>,

    /// Optional [augmentation] section
    augmentation: Option<FileAugmentation>,
}

impl Config {
    /// Get the config file path: ~/.config/anthropic-spy/config.toml
    /// Uses Unix-style ~/.config on all platforms for consistency
    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|p| p.join(".config").join("anthropic-spy").join("config.toml"))
    }

    /// Create config template if it doesn't exist
    /// Called during startup to help users discover configuration options
    pub fn ensure_config_exists() {
        let Some(path) = Self::config_path() else {
            return;
        };

        // Don't overwrite existing config
        if path.exists() {
            return;
        }

        // Create parent directory
        if let Some(parent) = path.parent() {
            if std::fs::create_dir_all(parent).is_err() {
                return; // Silently fail - config is optional
            }
        }

        let template = r#"# anthropic-spy configuration
# Uncomment and modify options as needed

# Theme: One Half Dark, Dracula, Nord, Gruvbox Dark, Monokai Pro, TokyoNight, etc.
# See full list in the theme selector (press 't' in the TUI)
# theme = "One Half Dark"

# Use theme's background color (true) or terminal's default (false)
# Set to false if you want the TUI to inherit your terminal's background
# use_theme_background = true

# Context window limit for the gauge (default: 147000)
# context_limit = 147000

# Proxy bind address (default: 127.0.0.1:8080)
# bind_addr = "127.0.0.1:8080"

# Log directory for session files (default: ./logs)
# log_dir = "./logs"

# Feature flags (default: all enabled)
# [features]
# storage = true         # Write events to JSONL files
# thinking_panel = true  # Show Claude's extended thinking
# stats = true           # Track token counts and costs

# Augmentation (response modifications)
# [augmentation]
# context_warning = true              # Inject warnings when context fills up (default: true)
# context_warning_thresholds = [60, 80, 85, 90, 95]  # Percentages to warn at
"#;

        // Write template (ignore errors - config is optional)
        let _ = std::fs::write(&path, template);
    }

    /// Load file config if it exists
    fn load_file_config() -> FileConfig {
        let Some(path) = Self::config_path() else {
            return FileConfig::default();
        };

        match std::fs::read_to_string(&path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                FileConfig::default()
            }),
            Err(_) => FileConfig::default(), // File doesn't exist, use defaults
        }
    }

    /// Load configuration: file -> env vars -> defaults
    pub fn from_env() -> Self {
        let file = Self::load_file_config();

        // Bind address: env > file > default
        let bind_addr = std::env::var("ANTHROPIC_SPY_BIND")
            .ok()
            .or(file.bind_addr)
            .unwrap_or_else(|| "127.0.0.1:8080".to_string())
            .parse()
            .expect("Invalid bind address");

        // API URL: env > file > default
        let api_url = std::env::var("ANTHROPIC_API_URL")
            .ok()
            .or(file.api_url)
            .unwrap_or_else(|| "https://api.anthropic.com".to_string());

        // Log directory: env > file > default
        let log_dir = std::env::var("ANTHROPIC_SPY_LOG_DIR")
            .ok()
            .or(file.log_dir)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("./logs"));

        // TUI toggle: env only (runtime flag)
        let enable_tui = std::env::var("ANTHROPIC_SPY_NO_TUI")
            .map(|v| v != "1" && v.to_lowercase() != "true")
            .unwrap_or(true);

        // Demo mode: env only (runtime flag)
        let demo_mode = std::env::var("ANTHROPIC_SPY_DEMO")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        // Context limit: env > file > default (147K based on empirical data)
        let context_limit = std::env::var("ANTHROPIC_SPY_CONTEXT_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .or(file.context_limit)
            .unwrap_or(147_000);

        // Theme: env > file > default ("One Half Dark" for consistent RGB colors)
        let theme = std::env::var("ANTHROPIC_SPY_THEME")
            .ok()
            .or(file.theme)
            .unwrap_or_else(|| "One Half Dark".to_string());

        // Use theme background: file > default (true = use theme's bg color)
        let use_theme_background = file.use_theme_background.unwrap_or(true);

        // Feature flags: file config only (env vars would be verbose)
        // Default: enabled (opt-out pattern)
        let file_features = file.features.unwrap_or_default();
        let features = Features {
            storage: file_features.storage.unwrap_or(true),
            thinking_panel: file_features.thinking_panel.unwrap_or(true),
            stats: file_features.stats.unwrap_or(true),
        };

        // Augmentation settings: file config only
        // Default: enabled (context warning is helpful and non-intrusive)
        let file_augmentation = file.augmentation.unwrap_or_default();
        let augmentation = Augmentation {
            context_warning: file_augmentation.context_warning.unwrap_or(true),
            context_warning_thresholds: file_augmentation
                .context_warning_thresholds
                .unwrap_or_else(|| vec![60, 80, 85, 90, 95]),
        };

        Self {
            bind_addr,
            api_url,
            log_dir,
            enable_tui,
            demo_mode,
            context_limit,
            theme,
            use_theme_background,
            features,
            augmentation,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:8080".parse().unwrap(),
            api_url: "https://api.anthropic.com".to_string(),
            log_dir: PathBuf::from("./logs"),
            enable_tui: true,
            demo_mode: false,
            context_limit: 147_000,
            theme: "One Half Dark".to_string(),
            use_theme_background: true,
            features: Features::default(),
            augmentation: Augmentation::default(),
        }
    }
}
