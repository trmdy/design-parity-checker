---
id: design-parity-checker-dpo
status: closed
deps: []
links: []
created: 2025-12-14T12:02:39.395868+01:00
type: feature
priority: 2
---
# Split browser.rs into modular directory structure

Split the 1049-LOC browser.rs into focused modules. Currently handles BrowserManager, Playwright scripts (inline JS), process spawning, timeout handling, and DOM conversion. Target: 4 modules under src/browser/.


