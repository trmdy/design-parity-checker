---
id: design-parity-checker-h0g
status: closed
deps: [design-parity-checker-4v9]
links: []
created: 2025-12-13T21:40:49.942146+01:00
type: task
priority: 0
---
# Create metrics trait/interface

Design pluggable metric interface: trait Metric { fn compute(ref: NormalizedView, impl: NormalizedView) -> MetricResult }. Enable selective metric execution via --metrics flag. Support future metric additions.


