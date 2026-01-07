---
id: design-parity-checker-6zo
status: closed
deps: [design-parity-checker-2j5]
links: []
created: 2025-12-13T21:41:38.008871+01:00
type: task
priority: 2
---
# Implement dpc quality command (experimental)

Reference-free quality scoring with flags: --input, --input-type, --viewport, --output, --format. Compute heuristic score [0,1] plus findings (alignment, spacing, contrast, hierarchy). Label as experimental.

## Notes

Implemented experimental quality scoring with heuristics on DOM/Figma/ocr, outputs normalized score + findings; added CLI tests.


