---
id: design-parity-checker-wex
status: closed
deps: [design-parity-checker-d8h]
links: []
created: 2025-12-13T21:40:49.517311+01:00
type: task
priority: 1
---
# Implement Typography Similarity metric

For matched elements, extract font-family, font-size, font-weight, line-height. Map families to canonical groups. Compute penalties: family mismatch (high), size diff >10% (medium), weight category mismatch (medium). Score = 1 - clamp(weighted_avg_penalty).


