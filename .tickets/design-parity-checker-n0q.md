---
id: design-parity-checker-n0q
status: closed
deps: [design-parity-checker-h0g]
links: []
created: 2025-12-13T21:40:49.34555+01:00
type: task
priority: 0
---
# Implement Pixel/Perceptual Similarity metric

Use image-compare crate (SSIM) or dssim for perceptual similarity. Convert screenshots to common color space, compute metric normalized to [0,1]. Generate diff heatmap and cluster hot pixels into regions with severity levels (minor/moderate/major).


