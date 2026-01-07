---
id: design-parity-checker-ccy
status: closed
deps: []
links: []
created: 2025-12-14T10:08:57.4295+01:00
type: task
priority: 2
---
# Soften metrics UX (layout fallback & issue ordering)

Layout metric should return low score + issue when impl has zero elements instead of hard error. Order generate_top_issues by severity/priority so palette shifts outrank minor typography. Add tests for new ordering and no-error layout fallback.

## Notes

Completed: summary uses generate_top_issues ordering; layout fallback surfaced; tests added


