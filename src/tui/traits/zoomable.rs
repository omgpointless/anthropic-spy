//! Zoomable trait for components that can expand to full content area
//!
//! When zoomed, a component takes the entire main content area (between
//! title bar and status bar). User presses 'z' to toggle zoom, Tab cycles
//! between zoomable panels while maintaining zoom state.
//!
//! # Design Philosophy
//!
//! Zoom is VIEW behavior - views decide HOW to render expanded.
//! This trait declares CAPABILITY - components say IF they can zoom.
//! App tracks STATE - whether zoom is currently active and which panel.
//!
//! # Example
//!
//! ```ignore
//! impl Zoomable for EventsPanel {
//!     fn zoom_label(&self) -> &'static str {
//!         "Events"
//!     }
//!
//!     // Optional: override to disable zoom
//!     // fn can_zoom(&self) -> bool { false }
//! }
//! ```

use super::Interactive;

/// Trait for components that support full-screen zoom
///
/// Components implementing this trait can expand to fill the entire
/// content area when the user presses 'z'. The zoom state is managed
/// by App, not the component itself.
///
/// # Zoom Behavior
///
/// - Press 'z': Toggle zoom for focused panel
/// - Tab while zoomed: Cycle to next zoomable panel (stays zoomed)
/// - Esc while zoomed: Exit zoom mode
/// - Press 'z' again: Exit zoom mode
pub trait Zoomable: Interactive {
    /// Whether this component supports zooming
    ///
    /// Default is `true`. Override to return `false` for components
    /// that shouldn't be zoomable (e.g., fixed-size previews, mini panels).
    ///
    /// # Example
    ///
    /// ```ignore
    /// // A mini-preview panel that shouldn't zoom
    /// fn can_zoom(&self) -> bool {
    ///     false
    /// }
    /// ```
    fn can_zoom(&self) -> bool {
        true
    }

    /// Label shown when this component is zoomed
    ///
    /// Displayed in the title bar (right side) to indicate which
    /// panel is currently expanded. Should be concise (single word).
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn zoom_label(&self) -> &'static str {
    ///     "Events"
    /// }
    /// ```
    fn zoom_label(&self) -> &'static str;
}
