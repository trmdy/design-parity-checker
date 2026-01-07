---
id: design-parity-checker-hgq
status: closed
deps: [design-parity-checker-ksu]
links: []
created: 2025-12-13T22:14:47.389641+01:00
type: task
priority: 1
---
# Unify viewport handling across CLI/config/types with early validation and single Viewport type

Unify viewport handling: one canonical Viewport type shared across CLI/config/normalized types; CLI eagerly parses WIDTHxHEIGHT with helpful errors and echoes the parsed viewport in verbose output. Ensure downstream capture/metrics receive the same Viewport instance. Acceptance: no duplicate viewport structs; invalid input surfaces actionable error; defaults remain 1440x900.


