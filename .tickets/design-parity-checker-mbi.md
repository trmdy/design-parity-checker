---
id: design-parity-checker-mbi
status: closed
deps: []
links: []
created: 2025-12-14T12:02:40.502524+01:00
type: feature
priority: 3
---
# Split figma.rs into modular directory structure

Split the 822-LOC figma.rs into focused modules. Currently handles API types, figma_to_normalized_view conversion, tree building, and letterbox transforms. Target: 4 modules under src/figma/. Lower priority as figma_client.rs already provides some separation.


