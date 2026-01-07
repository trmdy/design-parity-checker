---
id: design-parity-checker-d9s
status: closed
deps: []
links: []
created: 2025-12-14T10:08:31.956731+01:00
type: task
priority: 2
---
# Polish pretty output (Stripe-level summary)

Redesign pretty output: status badge, similarity vs threshold, top issues (max 5), metric table, artifact paths. Default to pretty for TTY unless --format json; keep JSON unchanged. Add tests for human layout and artifact listing; file outputs stay plain.


