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

/// Generate the annotation instruction to inject into the system prompt
/// This tells Claude to emit a context warning in the anthropic-spy annotation format
fn generate_context_warning(percent: u8, current_tokens: u64, limit: u64) -> String {
    let current_k = current_tokens / 1000;
    let limit_k = limit / 1000;

    format!(
        r#"

<context-warning>
Context usage is at {percent}% ({current_k}K/{limit_k}K tokens). Include this annotation in your response:

`★ anthropic-spy (context) ─────────────────────────────`
Context at {percent}% ({current_k}K/{limit_k}K). Consider `/compact` to free up space.
`─────────────────────────────────────────────────────────`
</context-warning>"#
    )
}

/// Check if we should inject a context warning and return the modified request body if so
/// Returns None if no injection needed, Some(modified_body) if injection was done
pub fn maybe_inject_context_warning(
    body_bytes: &[u8],
    context_state: &SharedContextState,
) -> Option<Vec<u8>> {
    // Lock context state and check if we should warn
    let mut ctx = context_state.lock().ok()?;
    let threshold = ctx.should_warn()?;

    // We should warn - get the data we need
    let percent = threshold;
    let current_tokens = ctx.current_tokens;
    let limit = ctx.limit;

    // Parse the request body as JSON
    let mut request: serde_json::Value = serde_json::from_slice(body_bytes).ok()?;

    // Generate the warning text
    let warning = generate_context_warning(percent, current_tokens, limit);

    // Inject into system prompt
    if let Some(system) = request.get_mut("system") {
        match system {
            serde_json::Value::String(s) => {
                // System is a string - append our warning
                s.push_str(&warning);
            }
            serde_json::Value::Array(arr) => {
                // System is an array of content blocks - append a text block
                arr.push(serde_json::json!({
                    "type": "text",
                    "text": warning
                }));
            }
            _ => return None, // Unexpected format
        }
    } else {
        // No system prompt - add one
        request["system"] = serde_json::Value::String(warning);
    }

    // Mark that we warned at this threshold
    ctx.mark_warned(threshold);

    // Serialize the modified request
    serde_json::to_vec(&request).ok()
}

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
        _ => format!("Context at {}% ({}K/{}K).", percent, current_k, limit_k),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ContextState;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_generate_context_warning() {
        let warning = generate_context_warning(85, 127500, 150000);
        assert!(warning.contains("85%"));
        assert!(warning.contains("127K/150K"));
        assert!(warning.contains("★ anthropic-spy"));
    }

    #[test]
    fn test_no_injection_below_threshold() {
        let ctx = Arc::new(Mutex::new(ContextState::new(150000)));
        ctx.lock().unwrap().update(60000, 0); // 40%

        let body = br#"{"model": "claude", "messages": []}"#;
        let result = maybe_inject_context_warning(body, &ctx);
        assert!(result.is_none());
    }

    #[test]
    fn test_injection_above_threshold() {
        let ctx = Arc::new(Mutex::new(ContextState::new(150000)));
        ctx.lock().unwrap().update(120000, 0); // 80%

        let body = br#"{"model": "claude", "messages": [], "system": "You are helpful."}"#;
        let result = maybe_inject_context_warning(body, &ctx);

        assert!(result.is_some());
        let modified = String::from_utf8(result.unwrap()).unwrap();
        assert!(modified.contains("anthropic-spy"));
        assert!(modified.contains("80%"));
    }

    #[test]
    fn test_no_double_warning() {
        let ctx = Arc::new(Mutex::new(ContextState::new(150000)));
        ctx.lock().unwrap().update(120000, 0); // 80%

        let body = br#"{"model": "claude", "messages": []}"#;

        // First call should inject
        let result1 = maybe_inject_context_warning(body, &ctx);
        assert!(result1.is_some());

        // Second call should not inject (already warned at 80%)
        let result2 = maybe_inject_context_warning(body, &ctx);
        assert!(result2.is_none());
    }

    #[test]
    fn test_warn_at_next_threshold() {
        let ctx = Arc::new(Mutex::new(ContextState::new(150000)));
        ctx.lock().unwrap().update(120000, 0); // 80%

        let body = br#"{"model": "claude", "messages": []}"#;

        // First call at 80%
        let result1 = maybe_inject_context_warning(body, &ctx);
        assert!(result1.is_some());

        // Increase to 85%
        ctx.lock().unwrap().update(127500, 0);

        // Should warn again at 85%
        let result2 = maybe_inject_context_warning(body, &ctx);
        assert!(result2.is_some());
        let modified = String::from_utf8(result2.unwrap()).unwrap();
        assert!(modified.contains("85%"));
    }
}
