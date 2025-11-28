// Startup module - displays banner and module loading status
//
// This module provides a professional startup experience showing:
// - Version info and branding
// - Configuration loaded from file
// - Module loading status with checkmarks

use crate::config::{Config, Features, VERSION};

/// ANSI color codes for terminal output
mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const CYAN: &str = "\x1b[36m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const MAGENTA: &str = "\x1b[35m";
}

/// Module loading result for display
pub struct ModuleStatus {
    pub name: &'static str,
    pub enabled: bool,
    pub description: &'static str,
}

/// Print the startup banner and module loading status
/// This runs before the TUI takes over the screen (or in headless mode)
pub fn print_startup(config: &Config) {
    use colors::*;

    // Banner
    println!();
    println!("  {BOLD}{CYAN}Anthropic Spy{RESET} {DIM}v{VERSION}{RESET}");
    println!("  {DIM}Observability proxy for Claude Code{RESET}");
    println!();

    // Config file status
    if let Some(path) = Config::config_path() {
        if path.exists() {
            println!("  {DIM}Config:{RESET} {GREEN}✓{RESET} {}", path.display());
        } else {
            println!("  {DIM}Config:{RESET} {DIM}(using defaults){RESET}");
        }
    }
    println!();

    // Module loading
    println!("  {DIM}Loading modules...{RESET}");

    let modules = get_module_status(config);
    for module in &modules {
        print_module_status(module);
    }

    println!();

    // Proxy info
    println!(
        "  {MAGENTA}▸{RESET} Proxy listening on {BOLD}{}{RESET}",
        config.bind_addr
    );
    if config.demo_mode {
        println!("  {YELLOW}▸{RESET} {YELLOW}Demo mode active{RESET} {DIM}(mock events){RESET}");
    }
    println!();
}

/// Get status of all modules based on config
fn get_module_status(config: &Config) -> Vec<ModuleStatus> {
    let Features {
        storage,
        thinking_panel,
        stats,
    } = &config.features;

    vec![
        ModuleStatus {
            name: "proxy",
            enabled: true, // Core, always on
            description: "HTTP interception",
        },
        ModuleStatus {
            name: "parser",
            enabled: true, // Core, always on
            description: "Event extraction",
        },
        ModuleStatus {
            name: "tui",
            enabled: config.enable_tui,
            description: "Terminal interface",
        },
        ModuleStatus {
            name: "storage",
            enabled: *storage,
            description: "JSONL logging",
        },
        ModuleStatus {
            name: "thinking",
            enabled: *thinking_panel && config.enable_tui,
            description: "Thinking panel",
        },
        ModuleStatus {
            name: "stats",
            enabled: *stats,
            description: "Token tracking",
        },
    ]
}

/// Print a single module's status
fn print_module_status(module: &ModuleStatus) {
    use colors::*;

    let (icon, style) = if module.enabled {
        (format!("{GREEN}✓{RESET}"), "")
    } else {
        (format!("{DIM}○{RESET}"), DIM)
    };

    println!(
        "    {icon} {style}{:<12}{RESET} {DIM}{}{RESET}",
        module.name, module.description
    );
}

/// Print a brief startup message for TUI mode (shown in logs)
/// Full banner is printed to stdout before TUI takes over
pub fn log_startup(config: &Config) {
    tracing::info!("Anthropic Spy v{} starting", VERSION);
    tracing::info!("Proxy: {}", config.bind_addr);

    let enabled: Vec<_> = get_module_status(config)
        .iter()
        .filter(|m| m.enabled)
        .map(|m| m.name)
        .collect();

    tracing::info!("Modules: {}", enabled.join(", "));
}
