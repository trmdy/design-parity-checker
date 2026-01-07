---
id: design-parity-checker-7oe
status: closed
deps: []
links: []
created: 2025-12-13T23:45:42.397288+01:00
type: task
priority: 2
---
# Add CLI exit-code integration smoke tests

Add small CLI-level integration tests to validate exit codes end-to-end: compare pass returns 0, threshold failure returns 1, fatal/config errors return 2. Use fixtures or minimal stubs (mock resources or temporary images) to avoid external dependencies. Document expected exit codes in test names and ensure coverage for JSON vs pretty output.

## Notes

CLI exit-code smoke tests implemented and passing


