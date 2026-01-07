---
id: design-parity-checker-ita
status: closed
deps: []
links: []
created: 2025-12-14T10:09:12.524429+01:00
type: task
priority: 2
---
# Make artifacts discoverable

Always print artifact directory on success/failure (unless suppressed) and include paths in pretty summary. Ensure JSON artifacts block is populated. Add a hint for viewing diff/DOM snapshots. Add tests for artifact path presence.

## Notes

Artifacts already logged and surfaced: eprintln directory+paths, pretty shows artifacts, JSON populated; tests cover kept/non-kept artifacts.


