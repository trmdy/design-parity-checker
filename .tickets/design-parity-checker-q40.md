---
id: design-parity-checker-q40
status: closed
deps: []
links: []
created: 2025-12-13T22:28:06.186857+01:00
type: bug
priority: 2
---
# Detect missing Playwright JS dependency

When Node is present but the JS helper cannot require('playwright') (module missing), the browser runner returns a generic config error. Detect this specific failure (stderr from script) and surface a clear instruction to install Playwright npm package. Add test exercising the mapping.


