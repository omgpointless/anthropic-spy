# aspy-mcp

MCP server for [Aspy](https://github.com/omgpointless/aspy) — exposes session stats, context window, and memory recall to Claude Code.

## Installation

```bash
claude mcp add aspy -- npx -y aspy-mcp
```

Or with a custom proxy URL:

```bash
claude mcp add aspy -e ASPY_URL=http://192.168.1.100:8080 -- npx -y aspy-mcp
```

## Requirements

- [Aspy](https://github.com/omgpointless/aspy) running locally (default: `http://127.0.0.1:8080`)
- Node.js 18+

## Available Tools

### Session Tools (Current Session)

| Tool | Description |
|------|-------------|
| `aspy_stats` | Session statistics — tokens, costs, tool calls |
| `aspy_events` | Recent events — tool calls, thinking, API usage |
| `aspy_window` | Context window gauge — % full, warning level |
| `aspy_sessions` | List all active sessions |

### Memory Tools (Cross-Session Recall)

| Tool | Description |
|------|-------------|
| `aspy_recall` | **PRIMARY** — Search all past sessions (semantic + keyword) |
| `aspy_recall_thinking` | Search Claude's past reasoning |
| `aspy_recall_prompts` | Search your past questions |
| `aspy_recall_responses` | Search Claude's past answers |

### Lifetime Tools

| Tool | Description |
|------|-------------|
| `aspy_lifetime` | All-time usage stats across all sessions |
| `aspy_embeddings` | Semantic search indexer status |

## Usage

```
"How many tokens have I used?"     → aspy_stats
"Am I running out of context?"     → aspy_window
"That thing about golf and nature" → aspy_recall (semantic handles fuzzy!)
"All-time usage"                   → aspy_lifetime
```

---

## For Claude: Tool Selection Guide

> This section is for you, Claude. Follow these guidelines.

### Quick Reference

| Need | Use |
|------|-----|
| Check context window % | `aspy_window` |
| Session token/cost summary | `aspy_stats` |
| Recent tool calls | `aspy_events` |
| **Recover lost context** | `aspy_recall` ← Use this for memory! |
| Find WHY something was decided | `aspy_recall_thinking` |
| All-time usage summary | `aspy_lifetime` |

### Memory Recall

`aspy_recall` is THE tool for recovering lost context. It handles:
- **Exact queries**: "ContextState refactor"
- **Fuzzy queries**: "that thing about golf and nature?"

It automatically uses semantic search if embeddings are enabled, falling back to keyword-only if not.

### Specialized Recall (When Needed)

If `aspy_recall` returns too much noise, narrow with:
- `aspy_recall_thinking` — "You noticed this pattern was meta..."
- `aspy_recall_prompts` — "I said something about exhaustive patterns..."
- `aspy_recall_responses` — "You recommended a hybrid ELM pattern..."

### Error Handling

| Error | Cause | Fix |
|-------|-------|-----|
| "Context is per-session" | No user ID | MCP handles this; check ANTHROPIC_API_KEY |
| "Cannot determine user identity" | No API key | Ensure ANTHROPIC_API_KEY is set |
| "No active session" | Session not found | Wait for first API call |

### Best Practices

1. **Don't poll window constantly** — Check when user mentions context or before big ops
2. **Use aspy_recall for memory** — It's the only search tool you need
3. **Trust the warning level** — 🟢 normal, 🟡 >70%, 🟠 >85%, 🔴 >95%

---

## Multi-Client Setup

For multiple Claude Code instances:

```bash
claude mcp add aspy -e ASPY_CLIENT_ID=dev-1 -- npx -y aspy-mcp
```

This matches your proxy URL path: `http://localhost:8080/dev-1/v1/messages`

## License

MIT
