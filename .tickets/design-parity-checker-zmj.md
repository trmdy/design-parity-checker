---
id: design-parity-checker-zmj
status: closed
deps: [design-parity-checker-se9]
links: []
created: 2025-12-13T21:41:38.375234+01:00
type: task
priority: 2
---
# Implement --ignore-regions flag

Load JSON file with manual bounding boxes to exclude from pixel comparison. Format: [{x, y, w, h}]. Mask regions before computing pixel metric.


