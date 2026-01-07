---
id: design-parity-checker-q8f
status: closed
deps: []
links: []
created: 2025-12-14T10:07:33.530893+01:00
type: bug
priority: 2
---
# Pretty error fallback goes to stderr

When write_output fails, render_error routes pretty output to stderr instead of stdout, so pretty JSON isn't available to callers. Adjust fallback to align with JSON branch or respect output flag.


