---
id: design-parity-checker-ixs
status: closed
deps: []
links: []
created: 2025-12-13T22:17:38.492642+01:00
type: chore
priority: 2
---
# Preflight check for Node/Playwright dependency

Our browser runner shells out to an embedded JS Playwright script (node -e ...), but we never verify that node and the playwright npm package are installed. When missing, users just see a generic config error from the helper. Add a preflight check (or friendlier error mapping) that detects missing node or playwright and surfaces actionable guidance; include tests/docs.


