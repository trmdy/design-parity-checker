---
id: design-parity-checker-9iw
status: closed
deps: []
links: []
created: 2025-12-14T12:01:54.422205+01:00
type: feature
priority: 1
---
# Split metrics.rs into modular directory structure

Split the 2747-LOC metrics.rs god module into a modular directory structure. This file currently handles all 5 metric implementations (pixel, layout, typography, color, content), scoring logic, issue generation, and image processing utilities. Target: 9 focused modules of ~200-500 LOC each under src/metrics/. This is the highest-impact split for parallelization.


