---
id: design-parity-checker-79l
status: closed
deps: []
links: []
created: 2025-12-14T10:09:27.436362+01:00
type: task
priority: 2
---
# Add --config file support

Wire --config <file> to load defaults (viewport, threshold, timeouts, weights). Merge CLI overrides last; validate and error clearly on bad config. Document in README/ci_cheatsheet. Add a temp-file test for config parsing.


