---
id: design-parity-checker-lo3
status: closed
deps: [design-parity-checker-dpo]
links: []
created: 2025-12-14T12:03:12.43336+01:00
type: task
priority: 2
---
# Extract BrowserManager to browser/manager.rs

Extract BrowserManager struct, BrowserOptions, PageRenderResult, and semaphore-based concurrency from browser.rs to browser/manager.rs. ~300 LOC.


