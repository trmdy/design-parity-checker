---
id: design-parity-checker-ac1
status: closed
deps: [design-parity-checker-n0q]
links: []
created: 2025-12-13T21:40:49.853335+01:00
type: task
priority: 2
---
# Implement diff region clustering

For pixel diffs, cluster adjacent 'hot' pixels into bounding box regions. Classify severity based on size and intensity. For layout diffs, generate regions for missing/extra/shifted/resized elements.


