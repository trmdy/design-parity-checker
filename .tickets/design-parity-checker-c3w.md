---
id: design-parity-checker-c3w
status: closed
deps: []
links: []
created: 2025-12-14T12:02:38.291976+01:00
type: feature
priority: 2
---
# Split main.rs into commands and pipeline modules

Split the 1370-LOC main.rs into focused modules. Currently handles CLI dispatch, resource normalization, ignore logic, diff heatmap generation, artifact persistence, and output formatting. Target: 7 modules under src/commands/ and src/pipeline/.


