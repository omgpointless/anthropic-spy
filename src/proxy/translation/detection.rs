//! Format detection - identifies API format from request characteristics
//!
//! Detects whether an incoming request is in OpenAI or Anthropic format
//! by examining the path, headers, and body structure.

use super::ApiFormat;
use axum::http::HeaderMap;

/// Detects API format from request characteristics
///
/// Detection priority:
/// 1. Path-based detection (most reliable)
/// 2. Header-based detection (Content-Type hints)
/// 3. Body structure detection (fallback)
#[derive(Debug, Clone)]
pub struct FormatDetector {
    /// Whether to auto-detect format (vs. assume Anthropic)
    auto_detect: bool,
}

impl FormatDetector {
    /// Create a detector with auto-detection enabled
    pub fn new() -> Self {
        Self { auto_detect: true }
    }

    /// Create a detector with specified auto-detection setting
    pub fn with_config(auto_detect: bool) -> Self {
        Self { auto_detect }
    }

    /// Detect the API format of a request
    ///
    /// # Arguments
    /// * `path` - Request path (e.g., "/v1/messages" or "/v1/chat/completions")
    /// * `headers` - Request headers
    /// * `body` - Request body bytes
    ///
    /// # Returns
    /// Detected API format (defaults to Anthropic if uncertain)
    pub fn detect(&self, path: &str, headers: &HeaderMap, body: &[u8]) -> ApiFormat {
        if !self.auto_detect {
            return ApiFormat::Anthropic;
        }

        // 1. Path-based detection (highest priority)
        if let Some(format) = self.detect_from_path(path) {
            tracing::trace!("Format detected from path '{}': {}", path, format);
            return format;
        }

        // 2. Header-based detection
        if let Some(format) = self.detect_from_headers(headers) {
            tracing::trace!("Format detected from headers: {}", format);
            return format;
        }

        // 3. Body structure detection (fallback)
        if let Some(format) = self.detect_from_body(body) {
            tracing::trace!("Format detected from body structure: {}", format);
            return format;
        }

        // Default to Anthropic (native format)
        tracing::trace!("No format detected, defaulting to Anthropic");
        ApiFormat::Anthropic
    }

    /// Detect format from request path
    fn detect_from_path(&self, path: &str) -> Option<ApiFormat> {
        // Strip any client prefix (e.g., /dev-1/v1/messages -> /v1/messages)
        let normalized_path = self.normalize_path(path);

        // OpenAI endpoints
        if normalized_path.contains("/chat/completions")
            || normalized_path.contains("/completions")
            || normalized_path.contains("/embeddings")
        {
            return Some(ApiFormat::OpenAI);
        }

        // Anthropic endpoints
        if normalized_path.contains("/messages") || normalized_path.contains("/complete") {
            return Some(ApiFormat::Anthropic);
        }

        None
    }

    /// Normalize path by stripping known prefixes
    fn normalize_path(&self, path: &str) -> String {
        // Remove leading slash
        let path = path.trim_start_matches('/');

        // Split into segments
        let segments: Vec<&str> = path.split('/').collect();

        // If first segment isn't a known API version prefix, it might be a client ID
        // e.g., "dev-1/v1/messages" -> "v1/messages"
        if !segments.is_empty() {
            let first = segments[0];
            // Known API prefixes
            if first == "v1" || first == "v2" || first == "api" {
                return format!("/{}", path);
            }
            // Otherwise, skip first segment (likely client ID)
            if segments.len() > 1 {
                return format!("/{}", segments[1..].join("/"));
            }
        }

        format!("/{}", path)
    }

    /// Detect format from request headers
    fn detect_from_headers(&self, headers: &HeaderMap) -> Option<ApiFormat> {
        // Check for OpenAI-specific headers
        if headers.contains_key("openai-organization")
            || headers.contains_key("openai-project")
            || headers
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.starts_with("Bearer sk-"))
                .unwrap_or(false)
        {
            return Some(ApiFormat::OpenAI);
        }

        // Check for Anthropic-specific headers
        if headers.contains_key("x-api-key")
            || headers.contains_key("anthropic-version")
            || headers.contains_key("anthropic-beta")
        {
            return Some(ApiFormat::Anthropic);
        }

        None
    }

    /// Detect format from body structure
    fn detect_from_body(&self, body: &[u8]) -> Option<ApiFormat> {
        // Try to parse as JSON
        let json: serde_json::Value = serde_json::from_slice(body).ok()?;
        let obj = json.as_object()?;

        // OpenAI indicators
        // - Has "messages" array with "content" as string (not array)
        // - Has no "system" field at top level
        // - Model names start with "gpt-" or "o1"
        if let Some(model) = obj.get("model").and_then(|m| m.as_str()) {
            if model.starts_with("gpt-")
                || model.starts_with("o1")
                || model.starts_with("text-")
                || model.starts_with("davinci")
                || model.starts_with("curie")
            {
                return Some(ApiFormat::OpenAI);
            }
            if model.starts_with("claude") {
                return Some(ApiFormat::Anthropic);
            }
        }

        // Check messages structure
        if let Some(messages) = obj.get("messages").and_then(|m| m.as_array()) {
            if let Some(first_msg) = messages.first().and_then(|m| m.as_object()) {
                // OpenAI allows system role in messages array
                if first_msg.get("role").and_then(|r| r.as_str()) == Some("system") {
                    // Anthropic would have this at top level as "system" field
                    return Some(ApiFormat::OpenAI);
                }

                // OpenAI content is typically a string
                // Anthropic content can be string or array of content blocks
                if let Some(content) = first_msg.get("content") {
                    if content.is_array() {
                        // Array content is more common in Anthropic format
                        if let Some(blocks) = content.as_array() {
                            // Check for Anthropic-specific content block types
                            for block in blocks {
                                if let Some(block_type) = block.get("type").and_then(|t| t.as_str())
                                {
                                    if matches!(block_type, "tool_use" | "tool_result" | "thinking")
                                    {
                                        return Some(ApiFormat::Anthropic);
                                    }
                                    if block_type == "image_url" {
                                        return Some(ApiFormat::OpenAI);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check for Anthropic-specific top-level fields
        if obj.contains_key("system") && obj.get("system").map(|s| s.is_string()).unwrap_or(false) {
            return Some(ApiFormat::Anthropic);
        }

        // Check for OpenAI-specific fields
        if obj.contains_key("frequency_penalty")
            || obj.contains_key("presence_penalty")
            || obj.contains_key("logprobs")
            || obj.contains_key("logit_bias")
            || obj.contains_key("n")
            || obj.contains_key("response_format")
            || obj.contains_key("seed")
            || obj.contains_key("user")
        {
            return Some(ApiFormat::OpenAI);
        }

        None
    }
}

impl Default for FormatDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_headers() -> HeaderMap {
        HeaderMap::new()
    }

    #[test]
    fn test_path_detection_openai() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        assert_eq!(
            detector.detect("/v1/chat/completions", &headers, b"{}"),
            ApiFormat::OpenAI
        );
        assert_eq!(
            detector.detect("/v1/completions", &headers, b"{}"),
            ApiFormat::OpenAI
        );
        assert_eq!(
            detector.detect("/v1/embeddings", &headers, b"{}"),
            ApiFormat::OpenAI
        );
    }

    #[test]
    fn test_path_detection_anthropic() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        assert_eq!(
            detector.detect("/v1/messages", &headers, b"{}"),
            ApiFormat::Anthropic
        );
        assert_eq!(
            detector.detect("/v1/complete", &headers, b"{}"),
            ApiFormat::Anthropic
        );
    }

    #[test]
    fn test_path_detection_with_client_prefix() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        // Client ID prefix should be stripped
        assert_eq!(
            detector.detect("/dev-1/v1/chat/completions", &headers, b"{}"),
            ApiFormat::OpenAI
        );
        assert_eq!(
            detector.detect("/dev-1/v1/messages", &headers, b"{}"),
            ApiFormat::Anthropic
        );
    }

    #[test]
    fn test_header_detection_openai() {
        let detector = FormatDetector::new();
        let mut headers = HeaderMap::new();
        headers.insert("openai-organization", "org-123".parse().unwrap());

        assert_eq!(
            detector.detect("/unknown", &headers, b"{}"),
            ApiFormat::OpenAI
        );
    }

    #[test]
    fn test_header_detection_anthropic() {
        let detector = FormatDetector::new();
        let mut headers = HeaderMap::new();
        headers.insert("anthropic-version", "2024-01-01".parse().unwrap());

        assert_eq!(
            detector.detect("/unknown", &headers, b"{}"),
            ApiFormat::Anthropic
        );
    }

    #[test]
    fn test_body_detection_openai_model() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        let body = br#"{"model": "gpt-4", "messages": []}"#;
        assert_eq!(
            detector.detect("/unknown", &headers, body),
            ApiFormat::OpenAI
        );

        let body = br#"{"model": "o1-preview", "messages": []}"#;
        assert_eq!(
            detector.detect("/unknown", &headers, body),
            ApiFormat::OpenAI
        );
    }

    #[test]
    fn test_body_detection_anthropic_model() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        let body = br#"{"model": "claude-sonnet-4-20250514", "messages": []}"#;
        assert_eq!(
            detector.detect("/unknown", &headers, body),
            ApiFormat::Anthropic
        );
    }

    #[test]
    fn test_body_detection_openai_specific_fields() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        let body = br#"{"model": "unknown", "frequency_penalty": 0.5}"#;
        assert_eq!(
            detector.detect("/unknown", &headers, body),
            ApiFormat::OpenAI
        );
    }

    #[test]
    fn test_body_detection_anthropic_system_field() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        let body = br#"{"model": "unknown", "system": "You are helpful", "messages": []}"#;
        assert_eq!(
            detector.detect("/unknown", &headers, body),
            ApiFormat::Anthropic
        );
    }

    #[test]
    fn test_auto_detect_disabled() {
        let detector = FormatDetector::with_config(false);
        let headers = make_headers();

        // Should always return Anthropic when auto-detect is off
        assert_eq!(
            detector.detect("/v1/chat/completions", &headers, b"{}"),
            ApiFormat::Anthropic
        );
    }

    #[test]
    fn test_default_to_anthropic() {
        let detector = FormatDetector::new();
        let headers = make_headers();

        // Ambiguous request should default to Anthropic
        assert_eq!(
            detector.detect("/api/unknown", &headers, b"{}"),
            ApiFormat::Anthropic
        );
    }
}
