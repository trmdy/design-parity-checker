---
id: design-parity-checker-d8h
status: closed
deps: [design-parity-checker-h0g]
links: []
created: 2025-12-13T21:40:49.43412+01:00
type: task
priority: 0
---
# Implement Layout/Structure Similarity metric

Extract elements from DOM/Figma (buttons, headings, text blocks, images, inputs). Normalize coords to [0,1] relative to viewport. Build spatial relation graph. Greedy match elements by type, label, proximity. Score = 0.5*match_rate + 0.5*avg_IoU.


