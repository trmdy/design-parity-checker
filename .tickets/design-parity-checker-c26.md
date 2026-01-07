---
id: design-parity-checker-c26
status: closed
deps: [design-parity-checker-c3w]
links: []
created: 2025-12-14T12:02:55.993584+01:00
type: task
priority: 2
---
# Extract compare command to commands/compare.rs

Extract compare command logic from main.rs run() to commands/compare.rs. Includes the full compare workflow: resource parsing, view normalization, metric execution, summary generation, artifact handling, and output rendering. ~500 LOC.


