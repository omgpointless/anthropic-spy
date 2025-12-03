//! OpenAI format translation
//!
//! This module provides bidirectional translation between OpenAI Chat Completions
//! API format and Anthropic Messages API format.
//!
//! # Supported Conversions
//!
//! - **Request**: OpenAI `/v1/chat/completions` → Anthropic `/v1/messages`
//! - **Response**: Anthropic SSE/JSON → OpenAI SSE/JSON

mod request;
mod response;

pub use request::OpenAiToAnthropicRequest;
pub use response::AnthropicToOpenAiResponse;
