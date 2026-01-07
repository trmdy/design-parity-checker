---
id: design-parity-checker-hjn
status: closed
deps: []
links: []
created: 2025-12-13T22:46:50.017922+01:00
type: task
priority: 2
---
# Add coverage for score aggregation and top issue ranking

calculate_combined_score and generate_top_issues lack unit tests for partial metric sets, zero weights, and severity ordering. Add focused tests to lock expected behavior (weights re-normalized over present metrics, issues sorted by severity, truncation respected).


