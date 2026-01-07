---
id: design-parity-checker-qpi
status: closed
deps: [design-parity-checker-4v9]
links: []
created: 2025-12-13T21:40:02.660836+01:00
type: task
priority: 1
---
# Implement Image to NormalizedView conversion

Load local images via Rust image crate (.png, .jpg, .jpeg, .webp, .gif). Handle size normalization: scale impl to ref resolution, letterbox if aspect ratio mismatch. Support --no-resize option.


