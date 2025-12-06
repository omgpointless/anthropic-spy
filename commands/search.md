---
description: "Search past sessions (thinking, prompts, responses)"
---

Use the `aspy_recall` MCP tool to search for: $ARGUMENTS

If no search terms were provided, ask the user: "What would you like to search for across your past sessions?"

This tool uses semantic search (if embeddings enabled) combined with keyword matching. It can handle:
- Exact queries: "ContextState refactor"
- Fuzzy queries: "that thing about golf and nature"

Show relevant matches with their source type (💭 thinking / 👤 prompt / 🤖 response) and session timestamp.
