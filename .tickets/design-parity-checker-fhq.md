---
id: design-parity-checker-fhq
status: closed
deps: []
links: []
created: 2025-12-14T10:07:46.130618+01:00
type: task
priority: 1
---
# Fix pretty output build & fallback

Add termcolor imports in src/main.rs; ensure colored pretty output compiles. When --output or non-tty, write plain pretty text; keep colors only for tty. Keep pretty errors on stderr with empty stdout. Add smoke test for plain-output path.


