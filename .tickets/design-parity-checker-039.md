---
id: design-parity-checker-039
status: closed
deps: []
links: []
created: 2025-12-14T10:08:30.441388+01:00
type: bug
priority: 2
---
# Normalize ignore-region coordinates to viewport

Ignore-region rectangles are interpreted after resizing screenshots to the viewport, so small coords (e.g., 4x4) do not cover the rendered 1440x900 image. Need to normalize region coordinates to the viewport/screenshot size or accept normalized percentages so masking matches the intended area without giant rectangles.


