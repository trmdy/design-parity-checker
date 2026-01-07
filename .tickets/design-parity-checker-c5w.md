---
id: design-parity-checker-c5w
status: closed
deps: []
links: []
created: 2025-12-13T22:46:34.497746+01:00
type: bug
priority: 2
---
# url_to_normalized_view returns opaque Playwright errors when npm module is missing

render_url maps the Playwright 'Cannot find module \'playwright\'' failure to an actionable config message, but url_to_normalized_view returns the raw JSON/exit text. Add consistent error mapping (and tests) so DOM capture path guides users to install playwright.


