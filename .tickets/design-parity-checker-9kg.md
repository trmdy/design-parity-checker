---
id: design-parity-checker-9kg
status: closed
deps: [design-parity-checker-bxg]
links: []
created: 2025-12-13T22:15:01.569012+01:00
type: task
priority: 1
---
# Wire Config into CLI flags + optional config file; expose defaults and effective settings

Wire Config into UX: map metric weights/timeouts/viewport to CLI flags and optional config file; show defaults in --help; print effective config in verbose runs; merge config file + CLI with clear precedence. Coordinate with viewport-unify (hgq) so a single Viewport flows through capture/metrics. Acceptance: flags + config file produce one Config used end-to-end; users can see and override defaults easily.


