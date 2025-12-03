//! OpenAI format translation
//!
//! This module provides bidirectional translation between OpenAI Chat Completions
//! API format and Anthropic Messages API format.
//!
//! # Supported Conversions
//!
//! ## Direction 1: OpenAI clients → Anthropic backend
//! - **Request**: OpenAI `/v1/chat/completions` → Anthropic `/v1/messages`
//! - **Response**: Anthropic SSE/JSON → OpenAI SSE/JSON
//!
//! ## Direction 2: Anthropic clients (Claude Code) → OpenAI backend
//! - **Request**: Anthropic `/v1/messages` → OpenAI `/v1/chat/completions`
//! - **Response**: OpenAI SSE/JSON → Anthropic SSE/JSON

mod request;
mod response;
mod reverse_request;
mod reverse_response;

// Direction 1: OpenAI → Anthropic
pub use request::OpenAiToAnthropicRequest;
pub use response::AnthropicToOpenAiResponse;

// Direction 2: Anthropic → OpenAI (reverse)
pub use reverse_request::AnthropicToOpenAiRequest;
pub use reverse_response::OpenAiToAnthropicResponse;
