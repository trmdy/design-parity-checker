---
id: design-parity-checker-e1i
status: closed
deps: [design-parity-checker-c3w]
links: []
created: 2025-12-14T12:03:01.542176+01:00
type: task
priority: 2
---
# Extract artifact handling to pipeline/artifacts.rs

Extract persist_compare_artifacts, generate_diff_heatmap, write_json_pretty, resolve_artifacts_dir from main.rs to pipeline/artifacts.rs. ~200 LOC.


