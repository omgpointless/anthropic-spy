// Interceptor module - request/response modification for context augmentation
//
// This module handles injecting context-aware annotations into API requests.
// When context usage exceeds thresholds (80%, 85%, 90%, 95%), we inject
// instructions that prompt Claude to emit a ★ anthropic-spy annotation
// warning the user about context limits.
//
// Architecture:
// - Interceptor reads SharedContextState to check current usage
// - If threshold exceeded and not yet warned, modifies request body
// - Injects annotation format into system prompt
// - Claude sees the instruction and emits the annotation naturally
// - Claude Code renders it as a styled box

use crate::SharedContextState;

/// Generate SSE events to inject a context warning annotation at end of response
/// Returns None if no injection needed, Some(sse_bytes) if we should inject
///
/// The injection creates a new content block with the annotation text, formatted
/// as valid SSE that Claude Code will render as a styled annotation box.
pub fn maybe_generate_sse_injection(
    context_state: &SharedContextState,
    next_block_index: u32,
) -> Option<Vec<u8>> {
    // Lock context state and check if we should warn
    let mut ctx = context_state.lock().ok()?;
    let threshold = ctx.should_warn()?;

    // We should warn - generate the SSE injection
    let percent = threshold;
    let current_k = ctx.current_tokens / 1000;
    let limit_k = ctx.limit / 1000;

    // Mark that we warned at this threshold
    ctx.mark_warned(threshold);

    // Tiered messaging - informative at low thresholds, actionable at high
    let message = match percent {
        95.. => format!(
            "Context at {}% ({}K/{}K). `/compact` recommended.",
            percent, current_k, limit_k
        ),
        85..=94 => format!(
            "Context at {}% ({}K/{}K). Consider `/compact` soon.",
            percent, current_k, limit_k
        ),
        80..=84 => format!("Context at {}% ({}K/{}K).", percent, current_k, limit_k),
        _ => format!(
            "Context at {}% ({}K/{}K). Halfway there.",
            percent, current_k, limit_k
        ),
    };

    // Build the annotation text
    let annotation = format!(
        "\n\n`★ anthropic-spy (context) ─────────────────────────────`\n\
         {}\n\
         `─────────────────────────────────────────────────────────`",
        message
    );

    // Generate SSE events for a new content block
    // IMPORTANT: SSE format requires "data:" at column 0, no leading whitespace
    let sse = format!(
        "event: content_block_start\ndata: {{\"type\":\"content_block_start\",\"index\":{idx},\"content_block\":{{\"type\":\"text\",\"text\":\"\"}}}}\n\nevent: content_block_delta\ndata: {{\"type\":\"content_block_delta\",\"index\":{idx},\"delta\":{{\"type\":\"text_delta\",\"text\":{text}}}}}\n\nevent: content_block_stop\ndata: {{\"type\":\"content_block_stop\",\"index\":{idx}}}\n\n",
        idx = next_block_index,
        text = serde_json::to_string(&annotation).unwrap_or_default()
    );

    tracing::info!(
        "Context: {}% ({}K/{}K) #{}",
        percent,
        current_k,
        limit_k,
        next_block_index
    );

    Some(sse.into_bytes())
}
