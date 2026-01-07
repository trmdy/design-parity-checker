---
id: design-parity-checker-woo
status: closed
deps: [design-parity-checker-6zo]
links: []
created: 2025-12-13T21:42:53.505882+01:00
type: task
priority: 2
---
# Implement spacing heuristic for quality mode

Analyze gaps between neighboring elements. Cluster spacing values. Flag outliers when 5+ distinct vertical spacing values detected. Report inconsistent spacing findings.

## Notes

Implemented spacing heuristic: detects many distinct vertical gaps, reports warning and adjusts score; tests added


