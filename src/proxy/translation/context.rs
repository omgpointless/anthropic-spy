//! Translation context - carries state from request to response translation
//!
//! The `TranslationContext` is created during request translation and passed
//! through to response translation. It contains metadata needed to correctly
//! convert responses back to the client's expected format.
//!
//! # Implementation Status
//!
//! This context supports both buffered and streaming response translation:
//!
//! - **Buffered (Integrated)**: Fields like `client_format`, `backend_format`,
//!   `original_model`, and `model_mapping` are used by `translate_buffered()`.
//!
//! - **Streaming (Infrastructure Ready)**: Fields like `line_buffer`, `chunk_index`,
//!   `completion_id`, and `finish_reason` are used by `translate_chunk()` in
//!   `openai/response.rs`. These are fully implemented but not yet called from
//!   `handle_streaming_response()` in `proxy/mod.rs`.
//!
//! The streaming fields are marked with `#[allow(dead_code)]` until proxy
//! integration is complete.

use super::ApiFormat;
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Model Mapping
// ============================================================================

/// Bidirectional model name mapping between API formats
///
/// Maps model names from one API format to another. For example:
/// - OpenAI `gpt-4` → Anthropic `claude-sonnet-4-20250514`
/// - Anthropic `claude-sonnet-4-20250514` → OpenAI `gpt-4` (reverse lookup)
#[derive(Debug, Clone, Default)]
pub struct ModelMapping {
    /// OpenAI model name → Anthropic model name
    openai_to_anthropic: HashMap<String, String>,
    /// Anthropic model name → OpenAI model name (reverse)
    anthropic_to_openai: HashMap<String, String>,
}

impl ModelMapping {
    /// Create empty mapping
    pub fn new() -> Self {
        Self::default()
    }

    /// Create mapping from config HashMap
    pub fn from_config(config: &HashMap<String, String>) -> Self {
        let mut mapping = Self::new();
        for (openai_model, anthropic_model) in config {
            mapping.add(openai_model.clone(), anthropic_model.clone());
        }
        mapping
    }

    /// Add a bidirectional mapping
    pub fn add(&mut self, openai_model: String, anthropic_model: String) {
        self.anthropic_to_openai
            .insert(anthropic_model.clone(), openai_model.clone());
        self.openai_to_anthropic
            .insert(openai_model, anthropic_model);
    }

    /// Map OpenAI model name to Anthropic
    ///
    /// Returns the mapped name, or the original if no mapping exists.
    pub fn to_anthropic(&self, openai_model: &str) -> String {
        self.openai_to_anthropic
            .get(openai_model)
            .cloned()
            .unwrap_or_else(|| {
                // Default mapping for common models
                match openai_model {
                    "gpt-4" | "gpt-4-turbo" | "gpt-4-turbo-preview" | "gpt-4o" => {
                        "claude-sonnet-4-20250514".to_string()
                    }
                    "gpt-4o-mini" | "gpt-3.5-turbo" | "gpt-3.5-turbo-16k" => {
                        "claude-3-haiku-20240307".to_string()
                    }
                    "o1" | "o1-preview" => "claude-sonnet-4-20250514".to_string(),
                    "o1-mini" => "claude-3-haiku-20240307".to_string(),
                    // Pass through unknown models (may work if Anthropic adds aliases)
                    _ => openai_model.to_string(),
                }
            })
    }

    /// Map Anthropic model name back to OpenAI
    ///
    /// Returns the mapped name, or a sensible default if no mapping exists.
    pub fn to_openai(&self, anthropic_model: &str) -> String {
        self.anthropic_to_openai
            .get(anthropic_model)
            .cloned()
            .unwrap_or_else(|| {
                // Default reverse mapping
                if anthropic_model.contains("opus") {
                    "gpt-4".to_string()
                } else if anthropic_model.contains("sonnet") {
                    "gpt-4-turbo".to_string()
                } else if anthropic_model.contains("haiku") {
                    "gpt-3.5-turbo".to_string()
                } else {
                    "gpt-4".to_string()
                }
            })
    }
}

// ============================================================================
// Translation Context
// ============================================================================

/// Context carried from request translation to response translation
///
/// This struct maintains state needed to correctly translate responses back
/// to the client's expected format. It's created during request translation
/// and passed through the proxy to response translation.
///
/// # Field Categories
///
/// ## Core Fields (Used by Buffered Translation - Integrated)
/// - `client_format`, `backend_format`: Determine if/how translation occurs
/// - `model_mapping`: Bidirectional model name conversion
/// - `original_model`: Preserves client's model name for response
/// - `streaming`: Indicates if client requested SSE streaming
///
/// ## Streaming State Fields (Used by Streaming Translation - Not Yet Integrated)
/// These fields are fully implemented and used by `translate_chunk()` in
/// `openai/response.rs`, but the integration into `handle_streaming_response()`
/// is pending. They track mutable state across SSE chunks:
///
/// - `line_buffer`: Handles SSE events split across TCP chunks
/// - `completion_id`: OpenAI's `chatcmpl-xxx` ID (generated once per request)
/// - `chunk_index`: Tracks content block index for tool calls
/// - `accumulated_content`: For potential usage calculation
/// - `sent_initial`: Ensures role is sent only in first chunk
/// - `finish_reason`: Captured from `message_delta` for final chunk
/// - `response_model`: Model name from Anthropic response (for mapping back)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TranslationContext {
    // ─────────────────────────────────────────────────────────────────────────
    // Core fields (used by buffered translation - INTEGRATED)
    // ─────────────────────────────────────────────────────────────────────────
    /// Original format the client spoke (e.g., OpenAI)
    pub client_format: ApiFormat,

    /// Format used with the backend (typically Anthropic)
    pub backend_format: ApiFormat,

    /// Model mapping for bidirectional name translation
    pub model_mapping: Arc<ModelMapping>,

    /// Original model name from client request (preserved for response mapping)
    pub original_model: Option<String>,

    /// Whether the client requested streaming (stream: true in request)
    pub streaming: bool,

    /// Unique request ID for correlation (optional, for logging)
    pub request_id: Option<String>,

    // ─────────────────────────────────────────────────────────────────────────
    // Streaming state fields (used by translate_chunk() - NOT YET INTEGRATED)
    //
    // These fields support the streaming translation logic in openai/response.rs.
    // The implementation is complete, but handle_streaming_response() in
    // proxy/mod.rs does not yet call translate_chunk(). Until then, these
    // fields are unused at runtime (hence #[allow(dead_code)] on the struct).
    // ─────────────────────────────────────────────────────────────────────────
    /// Buffer for incomplete SSE lines that span chunk boundaries
    ///
    /// SSE events may be split across TCP chunks. This buffer accumulates
    /// partial lines until a complete `data: {...}\n\n` is received.
    pub line_buffer: String,

    /// Generated completion ID for OpenAI format (e.g., "chatcmpl-abc123")
    ///
    /// Created once per request and reused across all streaming chunks.
    /// OpenAI clients expect the same ID throughout the stream.
    pub completion_id: String,

    /// Current content block index for tool call streaming
    ///
    /// OpenAI's streaming format requires an `index` field for tool calls.
    /// This increments when Anthropic sends `content_block_stop` events.
    pub chunk_index: u32,

    /// Accumulated content for potential usage calculation
    ///
    /// Some OpenAI clients expect usage stats even in streaming mode.
    /// Content is accumulated here for token counting if needed.
    pub accumulated_content: String,

    /// Whether the initial chunk with role has been sent
    ///
    /// OpenAI streaming sends `{"role": "assistant"}` only in the first chunk.
    /// This flag ensures we don't repeat it.
    pub sent_initial: bool,

    /// Finish reason captured from Anthropic's `message_delta` event
    ///
    /// Anthropic sends `stop_reason` in `message_delta`, which maps to
    /// OpenAI's `finish_reason` (e.g., "end_turn" → "stop").
    pub finish_reason: Option<String>,

    /// Model name from Anthropic response (may differ from request)
    ///
    /// Used for reverse mapping if `original_model` wasn't captured.
    pub response_model: Option<String>,
}

impl TranslationContext {
    /// Create a new translation context
    pub fn new(
        client_format: ApiFormat,
        backend_format: ApiFormat,
        model_mapping: Arc<ModelMapping>,
        streaming: bool,
    ) -> Self {
        Self {
            client_format,
            backend_format,
            model_mapping,
            original_model: None,
            streaming,
            request_id: None,
            line_buffer: String::new(),
            completion_id: generate_completion_id(),
            chunk_index: 0,
            accumulated_content: String::new(),
            sent_initial: false,
            finish_reason: None,
            response_model: None,
        }
    }

    /// Create a passthrough context (no translation needed)
    pub fn passthrough() -> Self {
        Self {
            client_format: ApiFormat::Anthropic,
            backend_format: ApiFormat::Anthropic,
            model_mapping: Arc::new(ModelMapping::new()),
            original_model: None,
            streaming: false,
            request_id: None,
            line_buffer: String::new(),
            completion_id: String::new(),
            chunk_index: 0,
            accumulated_content: String::new(),
            sent_initial: false,
            finish_reason: None,
            response_model: None,
        }
    }

    /// Check if response translation is needed
    pub fn needs_response_translation(&self) -> bool {
        self.client_format != self.backend_format
    }

    /// Set the original model name from the client request
    pub fn with_original_model(mut self, model: String) -> Self {
        self.original_model = Some(model);
        self
    }

    /// Set the request ID for correlation
    #[allow(dead_code)]
    pub fn with_request_id(mut self, id: String) -> Self {
        self.request_id = Some(id);
        self
    }

    /// Get the model name to use in responses
    ///
    /// Prefers the original model name from the request, falls back to
    /// mapping the response model, or returns a default.
    #[allow(dead_code)]
    pub fn response_model_name(&self) -> String {
        if let Some(ref original) = self.original_model {
            return original.clone();
        }
        if let Some(ref response) = self.response_model {
            return self.model_mapping.to_openai(response);
        }
        "gpt-4".to_string()
    }
}

impl Default for TranslationContext {
    fn default() -> Self {
        Self::passthrough()
    }
}

/// Generate a unique completion ID in OpenAI format
fn generate_completion_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    // Simple pseudo-random suffix using timestamp
    let suffix: u32 = (timestamp % 1_000_000) as u32;

    format!("chatcmpl-{:x}{:06x}", timestamp, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_mapping_defaults() {
        let mapping = ModelMapping::new();

        assert_eq!(mapping.to_anthropic("gpt-4"), "claude-sonnet-4-20250514");
        assert_eq!(
            mapping.to_anthropic("gpt-3.5-turbo"),
            "claude-3-haiku-20240307"
        );
        assert_eq!(mapping.to_anthropic("unknown-model"), "unknown-model");
    }

    #[test]
    fn test_model_mapping_custom() {
        let mut config = HashMap::new();
        config.insert("my-gpt".to_string(), "my-claude".to_string());

        let mapping = ModelMapping::from_config(&config);

        assert_eq!(mapping.to_anthropic("my-gpt"), "my-claude");
        assert_eq!(mapping.to_openai("my-claude"), "my-gpt");
    }

    #[test]
    fn test_model_mapping_reverse() {
        let mapping = ModelMapping::new();

        // Default reverse mappings
        assert_eq!(
            mapping.to_openai("claude-3-opus-20240229"),
            "gpt-4".to_string()
        );
        assert_eq!(
            mapping.to_openai("claude-sonnet-4-20250514"),
            "gpt-4-turbo".to_string()
        );
        assert_eq!(
            mapping.to_openai("claude-3-haiku-20240307"),
            "gpt-3.5-turbo".to_string()
        );
    }

    #[test]
    fn test_translation_context_passthrough() {
        let ctx = TranslationContext::passthrough();

        assert_eq!(ctx.client_format, ApiFormat::Anthropic);
        assert_eq!(ctx.backend_format, ApiFormat::Anthropic);
        assert!(!ctx.needs_response_translation());
    }

    #[test]
    fn test_translation_context_needs_translation() {
        let ctx = TranslationContext::new(
            ApiFormat::OpenAI,
            ApiFormat::Anthropic,
            Arc::new(ModelMapping::new()),
            true,
        );

        assert!(ctx.needs_response_translation());
    }

    #[test]
    fn test_completion_id_format() {
        let id = generate_completion_id();

        assert!(id.starts_with("chatcmpl-"));
        assert!(id.len() > 15); // Reasonable length
    }
}
