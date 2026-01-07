---
id: design-parity-checker-uvy
status: closed
deps: [design-parity-checker-2j5]
links: []
created: 2025-12-13T21:41:37.928707+01:00
type: task
priority: 2
assignee: BluePond
---
# Implement dpc generate-code command

Code generation command with flags: --input, --input-type, --stack html+tailwind (MVP only), --viewport, --output. Wrap screenshot-to-code or equivalent LLM pipeline. Return code only, errors as JSON.

## Notes

Implemented generate-code: normalize input via pipeline, emit HTML+Tailwind scaffold embedding screenshot plus notes; updated docs/tests.


