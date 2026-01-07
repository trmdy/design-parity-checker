---
id: design-parity-checker-gjs
status: closed
deps: [design-parity-checker-9iw]
links: []
created: 2025-12-14T12:02:19.198708+01:00
type: task
priority: 2
---
# Extract LayoutSimilarity metric to metrics/layout.rs

Extract LayoutSimilarity struct from metrics.rs to metrics/layout.rs. Includes element matching algorithm, IoU scoring, position/size diff detection. ~400 LOC.


