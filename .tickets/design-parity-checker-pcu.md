---
id: design-parity-checker-pcu
status: closed
deps: [design-parity-checker-4v9, design-parity-checker-a1o]
links: []
created: 2025-12-13T21:40:02.308265+01:00
type: task
priority: 0
---
# Implement URL to NormalizedView conversion

Use headless Chromium via Playwright to render web URLs at given viewport. Wait for network idle (configurable timeout). Capture PNG screenshot and DOM snapshot (node tree, bounding boxes, computed styles).

## Notes

ChartreusePond reports pcu complete; batch pushed and tests passing.


