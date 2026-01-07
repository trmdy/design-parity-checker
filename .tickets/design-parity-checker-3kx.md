---
id: design-parity-checker-3kx
status: closed
deps: [design-parity-checker-9iw]
links: []
created: 2025-12-14T12:02:18.232307+01:00
type: task
priority: 2
---
# Extract PixelSimilarity metric to metrics/pixel.rs

Extract PixelSimilarity struct and SSIM computation from metrics.rs to a dedicated metrics/pixel.rs module. Includes: PixelSimilarity impl, compute_ssim function, diff region clustering. ~500 LOC.


