---
id: design-parity-checker-ycm
status: closed
deps: [design-parity-checker-c3w]
links: []
created: 2025-12-14T12:03:00.247106+01:00
type: task
priority: 2
---
# Extract ignore logic to pipeline/ignore.rs

Extract apply_dom_ignores, apply_ignore_regions, load_ignore_regions, selector matching from main.rs to pipeline/ignore.rs. ~200 LOC.


