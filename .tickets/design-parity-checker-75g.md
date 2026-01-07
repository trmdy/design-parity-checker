---
id: design-parity-checker-75g
status: closed
deps: [design-parity-checker-c3w]
links: []
created: 2025-12-14T12:02:59.278262+01:00
type: task
priority: 2
---
# Extract resource normalization to pipeline/normalization.rs

Extract resource_to_normalized_view and mock_render_image_path from main.rs to pipeline/normalization.rs. Handles Image/URL/Figma resource → NormalizedView conversion. ~200 LOC.


