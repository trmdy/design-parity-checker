---
id: design-parity-checker-08r
status: closed
deps: []
links: []
created: 2025-12-13T22:31:26.119439+01:00
type: bug
priority: 0
---
# Fix main.rs API mismatches blocking build

main.rs has API mismatches: (1) image_to_normalized_view signature wrong (takes &str not &Path), (2) figma_to_normalized_view takes &FigmaClient and &FigmaRenderOptions, not individual args, (3) FigmaRenderOptions.viewport should be Option. Blocking full binary build.


