---
id: design-parity-checker-8h3
status: closed
deps: []
links: []
created: 2025-12-14T12:02:41.625688+01:00
type: feature
priority: 3
---
# Split types.rs by domain

Split the 278-LOC types.rs into domain-specific modules. Currently contains NormalizedView, DOM types, Figma types, and metric result types. Target: 4 modules under src/types/ for better organization as codebase grows.


