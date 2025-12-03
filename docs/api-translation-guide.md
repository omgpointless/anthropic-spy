---
layout: default
title: API Translation Guide
description: Configure and use Aspy's bidirectional API translation between OpenAI and Anthropic formats
---

# API Translation Guide

A comprehensive guide to Aspy's bidirectional API translation system—enabling any client to talk to any backend, regardless of API format.

## Overview

Aspy translates between OpenAI and Anthropic API formats **in both directions**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ DIRECTION 1: Claude Code → Other Models (PRIMARY USE CASE)                  │
│                                                                             │
│ Claude Code ──(Anthropic)──► Aspy ──(OpenAI)──► GPT-4/Azure/Ollama/etc     │
│             ◄──(Anthropic)──      ◄──(OpenAI)──                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│ DIRECTION 2: OpenAI Tools → Claude                                          │
│                                                                             │
│ LangChain/SDK ──(OpenAI)──► Aspy ──(Anthropic)──► Claude API               │
│               ◄──(OpenAI)──      ◄──(Anthropic)──                           │
└─────────────────────────────────────────────────────────────────────────────┘
```

**The killer feature**: Claude Code becomes a **universal AI coding interface**. Point it at any model.

---

## Use Cases

### Primary: Claude Code as Universal Interface

Claude Code speaks Anthropic format exclusively. With translation enabled:

| You Want To Use | How It Works |
|-----------------|--------------|
| **GPT-5.x** | Claude Code → Aspy → OpenAI API |
| **Gemini Pro 3** | Claude Code → Aspy → OpenRouter endpoint |
| **Azure OpenAI** | Claude Code → Aspy → Azure endpoint |
| **Ollama** | Claude Code → Aspy → `localhost:11434` |
| **Any OpenAI-compatible** | Claude Code → Aspy → that endpoint |

Claude Code's Anthropic requests get translated to OpenAI format, sent to your chosen backend, and responses translated back to Anthropic format.

### Secondary: OpenAI Tools → Claude

Existing OpenAI integrations can route through Aspy to Claude:

| Tool | Configuration |
|------|---------------|
| **LangChain** | Set `base_url` to Aspy |
| **OpenAI Python SDK** | `client = OpenAI(base_url="http://localhost:8080/dev-1")` |
| **Any `/v1/chat/completions` client** | Point at Aspy |

---

## Configuration

### Basic Setup

```toml
# ~/.config/aspy/config.toml

[translation]
enabled = true              # Master switch
auto_detect = true          # Auto-detect format from request
```

### Direction 1: Claude Code → OpenAI Backend

To route Claude Code requests to an OpenAI-compatible backend:

```toml
[translation]
enabled = true
auto_detect = true

# Map Anthropic models to OpenAI models
[translation.model_mapping]
"claude-sonnet-4-20250514" = "gpt-4"
"claude-3-haiku-20240307" = "gpt-3.5-turbo"

# Configure upstream (where to send translated requests)
[proxy]
upstream_url = "https://api.openai.com"  # Or Azure, Ollama, etc.
```

Then configure Claude Code:
```bash
export ANTHROPIC_BASE_URL=http://127.0.0.1:8080/dev-1
# Claude Code now routes through Aspy → OpenAI
```

### Direction 2: OpenAI Clients → Claude

To let OpenAI-format clients talk to Claude:

```toml
[translation]
enabled = true
auto_detect = true

# Map OpenAI models to Anthropic models
[translation.model_mapping]
"gpt-4" = "claude-sonnet-4-20250514"
"gpt-4-turbo" = "claude-sonnet-4-20250514"
"gpt-4o" = "claude-sonnet-4-20250514"
"gpt-3.5-turbo" = "claude-3-haiku-20240307"

# Upstream is Anthropic (default)
[proxy]
upstream_url = "https://api.anthropic.com"
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | bool | `false` | Master switch for translation |
| `auto_detect` | bool | `true` | Auto-detect API format from request |
| `model_mapping` | map | (see below) | Bidirectional model name mappings |

### Default Model Mappings

| OpenAI Model | Anthropic Model |
|--------------|-----------------|
| `gpt-4` | `claude-sonnet-4-20250514` |
| `gpt-4-turbo` | `claude-sonnet-4-20250514` |
| `gpt-4o` | `claude-sonnet-4-20250514` |
| `gpt-3.5-turbo` | `claude-3-haiku-20240307` |
| `o1` | `claude-sonnet-4-20250514` |
| `o1-mini` | `claude-3-haiku-20240307` |

Mappings work **bidirectionally**—the same config handles both directions.

---

## Format Detection

When `auto_detect = true`, Aspy determines the API format using:

1. **Path** (highest priority):
   - `/v1/chat/completions` → OpenAI format
   - `/v1/messages` → Anthropic format

2. **Headers**:
   - `openai-organization` header → OpenAI
   - `anthropic-version` header → Anthropic
   - `Bearer sk-...` auth → OpenAI
   - `x-api-key` header → Anthropic

3. **Body structure** (fallback):
   - Model prefix `gpt-`, `o1-` → OpenAI
   - Model prefix `claude` → Anthropic
   - OpenAI-specific fields (`frequency_penalty`, `logprobs`, `n`) → OpenAI
   - Anthropic-specific content types (`tool_use`, `thinking`) → Anthropic

---

## Request Translation

### Anthropic → OpenAI (Direction 1)

| Anthropic Parameter | OpenAI Equivalent | Notes |
|---------------------|-------------------|-------|
| `model` | `model` | Mapped via model_mapping |
| `messages` | `messages` | Content blocks flattened |
| `system` | `messages[0]` | Prepended as system message |
| `max_tokens` | `max_tokens` | Direct mapping |
| `temperature` | `temperature` | Scaled: Anthropic 0-1 → OpenAI 0-2 |
| `top_p` | `top_p` | Direct mapping |
| `stop_sequences` | `stop` | Direct mapping |
| `stream` | `stream` | Direct mapping |
| `tools` | `tools` | Structure adapted |

### OpenAI → Anthropic (Direction 2)

| OpenAI Parameter | Anthropic Equivalent | Notes |
|------------------|----------------------|-------|
| `model` | `model` | Mapped via model_mapping |
| `messages` | `messages` + `system` | System messages extracted |
| `max_tokens` | `max_tokens` | Default: 4096 if not specified |
| `temperature` | `temperature` | Scaled: OpenAI 0-2 → Anthropic 0-1 |
| `top_p` | `top_p` | Direct mapping |
| `stop` | `stop_sequences` | Converted to array |
| `stream` | `stream` | Direct mapping |
| `tools` | `tools` | Similar structure |
| `tool_choice` | `tool_choice` | Direct mapping |

### Ignored Parameters

These parameters are accepted but not translated (no equivalent in target format):
- `frequency_penalty` / `presence_penalty`
- `logprobs` / `top_logprobs`
- `n` (both APIs return 1 completion)
- `logit_bias`
- `thinking` (Anthropic-specific, filtered in translation)

---

## Response Translation

### Streaming Responses (`stream: true`)

**This is the critical path**—Claude Code uses `stream: true` for 95%+ of requests.

#### Anthropic → OpenAI (Direction 1)

| Anthropic Event | OpenAI Event |
|-----------------|--------------|
| `message_start` | Initial chunk with `role: "assistant"` |
| `content_block_start` (text) | (no event, wait for delta) |
| `content_block_start` (tool_use) | Tool call header with id, name |
| `content_block_delta` (text) | `choices[].delta.content` |
| `content_block_delta` (input_json) | Tool arguments streaming |
| `content_block_delta` (thinking) | **Filtered out** |
| `content_block_stop` | (internal index tracking) |
| `message_delta` | `choices[].finish_reason` |
| `message_stop` | `data: [DONE]` |

#### OpenAI → Anthropic (Direction 2)

| OpenAI Event | Anthropic Event |
|--------------|-----------------|
| Initial chunk with `role` | `message_start` |
| `delta.content` | `content_block_delta` (text_delta) |
| `delta.tool_calls` | `content_block_start` + deltas |
| `finish_reason` | `message_delta` with `stop_reason` |
| `data: [DONE]` | `message_stop` |

### Buffered Responses (`stream: false`)

Complete response translated at once. Output matches target format's structure.

### Stop Reason Mapping

| Anthropic | OpenAI |
|-----------|--------|
| `end_turn` | `stop` |
| `tool_use` | `tool_calls` |
| `max_tokens` | `length` |
| `stop_sequence` | `stop` |

---

## Usage Examples

### Example 1: Claude Code → GPT-5.x

```toml
# ~/.config/aspy/config.toml
[translation]
enabled = true
auto_detect = true

[translation.model_mapping]
"claude-sonnet-4-20250514" = "gpt-5.1"
"claude-3-haiku-20240307" = "gpt-4o-mini"

[proxy]
upstream_url = "https://api.openai.com"
```

```bash
# Terminal 1: Start Aspy
aspy

# Terminal 2: Configure Claude Code
export ANTHROPIC_BASE_URL=http://127.0.0.1:8080/dev-1
export OPENAI_API_KEY=sk-...  # Your OpenAI key

# Now use Claude Code normally - requests go to GPT-5.1
```

### Example 2: Claude Code → Ollama (Local)

```toml
# ~/.config/aspy/config.toml
[translation]
enabled = true
auto_detect = true

[translation.model_mapping]
"claude-sonnet-4-20250514" = "llama2"
"claude-3-haiku-20240307" = "codellama"

[proxy]
upstream_url = "http://localhost:11434"
```

```bash
# Start Ollama
ollama serve

# Start Aspy
aspy

# Configure Claude Code
export ANTHROPIC_BASE_URL=http://127.0.0.1:8080/dev-1
# Claude Code now talks to your local Ollama models
```

### Example 3: OpenAI SDK → Claude

```python
from openai import OpenAI

# Point at Aspy instead of OpenAI
client = OpenAI(
    base_url="http://localhost:8080/dev-1/v1",
    api_key="your-anthropic-key"  # Anthropic key, not OpenAI
)

response = client.chat.completions.create(
    model="gpt-4",  # Gets mapped to Claude
    messages=[{"role": "user", "content": "Hello!"}],
    stream=True
)

for chunk in response:
    print(chunk.choices[0].delta.content, end="")
```

### Example 4: Test with curl (OpenAI format → Claude)

```bash
# Start Aspy
aspy

# Send OpenAI-format request
curl http://localhost:8080/dev-1/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ANTHROPIC_API_KEY" \
  -d '{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello!"}],
    "stream": true
  }'
```

**Expected output** (OpenAI SSE format):
```
data: {"id":"chatcmpl-...","object":"chat.completion.chunk","created":...,"model":"gpt-4","choices":[{"index":0,"delta":{"role":"assistant"},"finish_reason":null}]}

data: {"id":"chatcmpl-...","object":"chat.completion.chunk","created":...,"model":"gpt-4","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}

...

data: [DONE]
```

---

## Verification Checklist

| Check | Command | Expected |
|-------|---------|----------|
| Config loaded | Check startup banner | Shows `✓ translation` in Pipeline |
| Buffered works | curl with `stream: false` | Returns target format JSON |
| Streaming works | curl with `stream: true` | Returns target format SSE |
| Model preserved | Check response | Returns same model name sent |
| Tools work | curl with tools | Tool calls in target format |

---

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| `○ translation (disabled)` in startup | `enabled = false` | Set `enabled = true` in config |
| Request not translated | Format not detected | Check path matches expected format |
| Response in wrong format | Detection failed | Ensure `auto_detect = true` |
| Model name wrong in response | Missing mapping | Add to `[translation.model_mapping]` |
| Streaming incomplete | TCP fragmentation | (Should work - report bug if not) |
| Tool calls missing | Different structure | Check tool format for target API |
| Claude Code not connecting | Wrong env var | Ensure `ANTHROPIC_BASE_URL` set |

---

## Architecture Notes

### How It Fits Together

| System | Purpose | When Runs |
|--------|---------|-----------|
| **Translation** | Format conversion (OpenAI ↔ Anthropic) | Pre/post proxy |
| **Augmentation** | Inject content into SSE streams | During streaming |
| **EventProcessor** | Transform parsed ProxyEvents | Post-parsing |

### Processing Order (Streaming)

```
1. Request arrives (any format)
2. Format detected
3. Request translated (if needed)
4. Forwarded to upstream
5. Response chunk arrives
6. Real-time extraction (RAW format for tool registration, thinking streaming)
7. Augmentation injection (in upstream format)
8. Translation to client format (if needed)
9. Forward to client
```

Translation happens at the **OUTPUT stage**, preserving internal observability. Aspy always sees the raw format internally, regardless of what clients send/receive.

### Model Name Preservation

The `TranslationContext` carries `original_model` through the request-response cycle:

```
Client: "gpt-4" → Aspy captures → Backend: "claude-sonnet-4" → Response: "gpt-4"
```

The client always sees the model name it originally requested.

---

## Quick Reference

```bash
# Enable translation
cat >> ~/.config/aspy/config.toml << 'EOF'
[translation]
enabled = true
auto_detect = true
EOF

# Custom model mapping (both directions)
cat >> ~/.config/aspy/config.toml << 'EOF'
[translation.model_mapping]
"gpt-4" = "claude-sonnet-4-20250514"
"claude-sonnet-4-20250514" = "gpt-4"
EOF

# Test OpenAI → Claude
curl http://localhost:8080/dev-1/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ANTHROPIC_API_KEY" \
  -d '{"model": "gpt-4", "messages": [{"role": "user", "content": "Hi"}]}'

# Configure Claude Code → other models
export ANTHROPIC_BASE_URL=http://127.0.0.1:8080/dev-1
```
