# Features

A detailed look at what Aspy can do.

## Real-time Thinking Panel

<!-- TODO: Add screenshot/gif -->

Watch Claude's reasoning stream word-by-word as it thinks through problems. The dedicated thinking panel shows extended thinking blocks in real-time, not after the fact.

- Streams incrementally as tokens arrive
- Dedicated panel keeps thinking visible while events scroll
- Supports markdown formatting with syntax highlighting

## Stats Dashboard

<!-- TODO: Add screenshot -->

Session analytics at a glance with ratatui widgets:

- **Bar charts** — Token distribution (cached vs input vs output)
- **Gauges** — Context window usage with color-coded thresholds
- **Sparklines** — Token usage trends over time
- **Tool breakdown** — Call counts and average durations

Press `s` to switch to Stats view, `Tab` to cycle through tabs.

## Context Warnings

<!-- TODO: Add screenshot -->

Automatic notifications when your context window fills up:

```
⚠️ Context at 80% - consider using /compact
```

Configurable thresholds (default: 60%, 80%, 85%, 90%, 95%) inject helpful reminders into Claude's responses suggesting when to compact.

```toml
[augmentation]
context_warning = true
context_warning_thresholds = [60, 80, 85, 90, 95]
```

## Theme System

<!-- TODO: Add screenshot of theme selector -->

32 bundled themes plus custom TOML support:

**Bundled themes include:**
- Spy Dark / Spy Light (flagship)
- Dracula, Nord, Gruvbox, Monokai Pro
- Catppuccin (Mocha, Latte)
- Tokyo Night, Synthwave '84
- And many more...

**Custom themes:** Drop a `.toml` file in `~/.config/aspy/themes/` with your colors.

Press `F3` for Settings, navigate to theme, press `Enter` to apply. Changes persist to config.

See [Themes documentation](themes.md) for creating custom themes.

## Multi-Client Routing

<!-- TODO: Add diagram -->

Track multiple Claude Code instances through a single proxy:

```toml
[clients.dev-1]
name = "Dev Laptop"
provider = "anthropic"

[clients.work]
name = "Work Projects"
provider = "anthropic"

[providers.anthropic]
base_url = "https://api.anthropic.com"
```

Connect via URL path:
```bash
export ANTHROPIC_BASE_URL=http://127.0.0.1:8080/dev-1
claude
```

Each client gets isolated session tracking. Query specific clients via API:
```bash
curl http://127.0.0.1:8080/api/stats?client=dev-1
```

See [Multi-Client Routing](sessions.md) for full configuration.

## Structured Logs

JSON Lines format for easy analysis:

```bash
# Count tool calls by type
jq -r 'select(.type=="tool_call") | .tool_name' logs/*.jsonl | sort | uniq -c

# Find slow tool calls (>5s)
jq 'select(.type=="tool_result" and .duration.secs > 5)' logs/*.jsonl

# Calculate cache efficiency
jq -s '[.[] | select(.type=="ApiUsage")] |
  (map(.cache_read) | add) as $cached |
  (map(.input_tokens) | add) as $input |
  {cache_ratio: (($cached / ($cached + $input)) * 100)}' logs/*.jsonl
```

See [Log Analysis](log-analysis.md) for more queries.

## REST API

Programmatic access to session data:

| Endpoint | Description |
|----------|-------------|
| `GET /api/stats` | Session statistics |
| `GET /api/events` | Recent events |
| `GET /api/context` | Context window status |
| `GET /api/sessions` | All tracked sessions |
| `POST /api/search` | Search past logs |

All endpoints support `?client=<id>` for multi-client filtering.

See [API Reference](api-reference.md) for full documentation.

## MCP Integration

Query session data from within Claude Code:

```bash
claude mcp add aspy -- npx -y aspy-mcp
```

Available tools:
- `aspy_stats` — Token counts, costs, cache efficiency
- `aspy_events` — Recent tool calls and results
- `aspy_context` — Context window percentage and warnings
- `aspy_search` — Search past session logs

## Keyboard Navigation

| Key | Action |
|-----|--------|
| `e` / F1 | Events view |
| `s` / F2 | Stats view |
| F3 | Settings view |
| `↑`/`↓` or `j`/`k` | Navigate |
| `Enter` | Open detail / Apply |
| `Escape` | Close / Back |
| `Tab` | Cycle focus / tabs |
| `q` | Quit |
