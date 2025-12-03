# Agent Guidelines

**Read `CLAUDE.md` first.** This file provides supplementary quick-reference for AI agents.

## Quick Decision Tree

**Where does new code go?**
1. Does the app work without it? No → Core | Yes → Extension
2. UI behavior multiple components need? → `tui/traits/[name].rs`
3. Reusable UI widget? → `tui/components/[name]_panel.rs`
4. Full-screen layout? → `tui/views/[name].rs`
5. Stream modification? → `proxy/augmentation/` or `proxy/transformation/` + config
6. Feature-local helper? → `[feature]/helpers/[name].rs`

**Where does state live?**
- Component-specific → inside component struct
- Cross-cutting → `app.rs`
- Extension state (augmentors) → inside the extension struct, NOT global

## Common Scenarios

**Adding scrolling to a panel:**
1. Add `scroll: ScrollState` field
2. `impl Scrollable for YourPanel`
3. Handle in `tui/mod.rs`: `panel.as_scrollable_mut()`

**Adding an augmentor:**
1. Create `proxy/augmentation/[name].rs`
2. `impl Augmenter` trait
3. Register in pipeline builder
4. Add config flag

**Adding a panel:**
1. Create `tui/components/[name]_panel.rs`
2. Implement needed traits (Scrollable, Copyable, etc.)
3. Add to view in `tui/views/`

## Pre-Commit Checklist

- [ ] No component state added to `app.rs`
- [ ] Components own state; views compose
- [ ] Userland features have config toggles
- [ ] `cargo fmt && cargo clippy`
- [ ] Tests pass

## References

See `CLAUDE.md` for architecture philosophy, mental model, anti-patterns, and documentation index.
