---
name: context-recovery
description: >
  Recover lost context after session compaction or when information from
  previous sessions is needed. Use when: user mentions "what were we working on",
  "I lost context", "before the compact", "previous session", or asks about
  decisions/implementations/discussions that aren't in current context.
  Also use proactively when you notice references to prior work you lack context for.
allowed-tools: Read, Grep, mcp__plugin_aspy_aspy__aspy_lifestats_context_hybrid, mcp__plugin_aspy_aspy__aspy_lifestats_context, mcp__plugin_aspy_aspy__aspy_search, mcp__plugin_aspy_aspy__aspy_lifestats_search_thinking, mcp__plugin_aspy_aspy__aspy_lifestats_search_prompts, mcp__plugin_aspy_aspy__aspy_lifestats_search_responses
---

# Context Recovery

You've been activated to recover context that was lost to compaction or exists in a previous session.

## Quick Start

1. **Identify the topic** - What specific context is needed?
   - If the user's request is vague, ask: "What topic should I search for?"

2. **Use hybrid search first** (best results):
   ```
   aspy_lifestats_context_hybrid(topic="<keywords>", limit=10)
   ```
   This combines semantic vector search with FTS5 keyword matching using RRF ranking.
   Searches thinking blocks, user prompts, AND assistant responses simultaneously.

3. **Synthesize, don't dump** - Summarize findings:
   - What was decided or implemented
   - Key file paths and line numbers mentioned
   - Any unfinished work or next steps discussed

4. **Offer continuity** - "Would you like me to continue where we left off?"

## Search Strategy

### Start with Hybrid (Best Quality)
- `aspy_lifestats_context_hybrid` combines semantic + keyword search
- Finds conceptually related content even with different wording
- Use specific keywords from the user's question
- Default limit of 10 results is usually sufficient

### Fallback to FTS-Only
- If hybrid returns insufficient results, try `aspy_lifestats_context`
- Use `mode: "natural"` for OR-style searches: `"auth OR authentication OR login"`
- Increase limit to 20-30 for comprehensive searches

### Targeted Searches (If Combined Is Noisy)
- `aspy_lifestats_search_thinking` - Claude's reasoning and analysis
- `aspy_lifestats_search_prompts` - What the user asked
- `aspy_lifestats_search_responses` - Claude's answers and code

## What Makes Good Context Recovery

**Good synthesis:**
> "On Dec 2nd, we implemented mouse scroll support for the detail modal.
> The fix was in `src/tui/mod.rs:299-322` - checking if modal is open
> before dispatching scroll events. You mentioned wanting to test it
> before merging."

**Bad synthesis:**
> "Found 5 results mentioning 'scroll'. Here they are: [dumps raw results]"

## Common Patterns

| User Says | Search For |
|-----------|------------|
| "that bug we fixed" | error keywords, "fix", file names |
| "the refactor" | "refactor", component names |
| "what we decided" | "decided", "approach", "pattern" |
| "before compact" | recent topics, use `time_range: "today"` |
