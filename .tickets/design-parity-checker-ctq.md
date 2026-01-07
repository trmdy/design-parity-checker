---
id: design-parity-checker-ctq
status: closed
deps: [design-parity-checker-9iw]
links: []
created: 2025-12-14T12:02:21.204594+01:00
type: task
priority: 2
---
# Extract ColorPaletteMetric to metrics/color.rs

Extract ColorPaletteMetric struct and k-means clustering from metrics.rs to metrics/color.rs. Includes: k-means algorithm, LAB color conversions, delta-E computation, palette extraction. ~500 LOC.


