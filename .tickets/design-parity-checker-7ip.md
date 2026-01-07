---
id: design-parity-checker-7ip
status: closed
deps: []
links: []
created: 2025-12-13T22:46:03.887819+01:00
type: bug
priority: 2
---
# Figma figma_to_normalized_view ignores requested viewport dimensions

figma_to_normalized_view accepts an optional viewport but never uses it; returned NormalizedView width/height always come from the exported PNG dimensions, so callers cannot request a specific viewport or scaled dimensions. Respect the viewport or document that it is ignored, and add tests to lock behavior.


