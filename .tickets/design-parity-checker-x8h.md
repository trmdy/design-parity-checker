---
id: design-parity-checker-x8h
status: closed
deps: [design-parity-checker-6zo]
links: []
created: 2025-12-13T21:42:20.73811+01:00
type: task
priority: 2
---
# Implement quality mode output format

For dpc quality command, output: version, mode, input object, viewport, score, findings array with severity (warning/info), type (alignment_inconsistent, spacing_inconsistent, low_contrast, missing_hierarchy), message.

## Notes

Quality output now emits score + findings with severity/info and types (alignment_inconsistent/spacing_inconsistent/low_contrast/missing_hierarchy) via heuristics; tests updated.


