---
id: design-parity-checker-123
status: closed
deps: [design-parity-checker-2lc]
links: []
created: 2025-12-13T22:14:28.728469+01:00
type: task
priority: 1
---
# CLI UX scaffolding & graceful fallbacks (json/pretty outputs, progress hints, non-implemented gating)

Deliver user-friendly CLI UX scaffolding before full features land: (1) Non-implemented commands return structured JSON with status, message, and next-steps; pretty mode mirrors JSON fields; (2) Verbose runs show staged progress (parse args, detect resources, prepare capture, run metrics) even when stubbed; (3) Exit codes documented/consistent. This is a temporary but polished user-facing surface so the CLI feels premium while core features are built.

## Notes

Stub UX polished; docs examples added; compare tests green


