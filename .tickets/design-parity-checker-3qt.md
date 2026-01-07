---
id: design-parity-checker-3qt
status: closed
deps: []
links: []
created: 2025-12-13T22:17:50.771689+01:00
type: bug
priority: 1
---
# Figma image download uses unauthenticated reqwest::get

Figma client builds an authenticated reqwest client but download_image bypasses it with bare reqwest::get, losing headers/timeouts/proxies. If the signed image URL requires auth or tighter timeouts, downloads may fail silently. Fix by reusing the configured client with per-request timeouts and add tests for error propagation.


