---
id: design-parity-checker-ife
status: closed
deps: [design-parity-checker-dpo]
links: []
created: 2025-12-14T12:03:13.405594+01:00
type: task
priority: 2
---
# Extract Playwright scripts to browser/playwright.rs

Extract PLAYWRIGHT_SCRIPT, PLAYWRIGHT_SCRIPT_WITH_DOM constants, run_playwright method, ensure_node/playwright_available, and error mapping from browser.rs to browser/playwright.rs. Consider also extracting JS to external files. ~400 LOC.


