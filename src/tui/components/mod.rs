// Components module - reusable UI building blocks
//
// Phase 1: Re-exports shell components from ui.rs (temporary bridge)
// Phase 2: Each component gets its own file
//
// Shell components (rendered in every view):
// - Title bar: App name, streaming indicator, topic
// - Status bar: Uptime, requests, tools, cost
// - Context bar: Context window usage gauge
// - Logs panel: System log entries

// Re-export shell components from ui.rs until Phase 2 extraction
pub use super::ui::{render_context_bar, render_logs_panel, render_status, render_title};
