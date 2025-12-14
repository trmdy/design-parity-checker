# Reporting & Outputs

Reference for what the CLI prints and how to consume it in pipelines.

## Compare output
- Format: `json` (default) or `pretty`.
- Fields (JSON): `mode` (`Compare`), `version`, `ref_resource`, `impl_resource`, `viewport`, `similarity`, `threshold`, `passed`, `metrics` (pixel/layout/typography/color/content), `summary.top_issues`.
- Pretty example:
```
Compare v0.2.0 (schema version)
ref:   Url https://example.com/design
impl:  Url https://example.com/build
viewport: 1440x900
similarity: 96.50% (threshold 95.00%) -> PASSED
Top issues:
- Design parity check passed (96.5% similarity, threshold: 95.0%)
Metrics:
  pixel: 0.965
  layout: 0.981
  typography: 0.942
  color: 0.973
  content: 0.955
```

## Errors
- Serialized as `{ "category": "<config|network|figma|image|metric|unknown>", "message": "...", "remediation": "..."? }` in JSON mode; pretty prints category and hint to stderr.
- Common causes: missing Playwright (`npm install playwright && npx playwright install chromium`), missing `FIGMA_TOKEN`, invalid viewport (`WIDTHxHEIGHT`).

## Exit codes
- `0`: compare passed (similarity >= threshold) or stub commands succeeded.
- `1`: compare failed threshold.
- `2`: configuration/network/runtime errors.

## Artifacts
- Written under `tmp/dpc-<pid>/` by default: `ref_screenshot.png`, `impl_screenshot.png`, DOM snapshots, and Figma exports.
- Use `--keep-artifacts` to retain; otherwise cleaned up after compare.
- Mocking (offline/CI): set `DPC_MOCK_RENDER_REF` / `DPC_MOCK_RENDER_IMPL` to PNGs, or `DPC_MOCK_RENDERERS_DIR=/path` containing `ref.png` / `impl.png`. Mocking only applies to URL/Figma kinds.

## Metrics weights
- Combined score weights: pixel 0.35, layout 0.25, typography 0.15, color 0.15, content 0.10. Only available metrics contribute.

## Figma & browser notes
- Figma: requires `FIGMA_TOKEN`; URLs must include `node-id`.
- Browser: Node on PATH + Playwright package; headless by default; waits for navigation and `networkidle`.

## Integration tips
- Prefer JSON for automation; pretty for humans.
- Treat exit code 1 as a validation failure (non-fatal) and 2 as infra/config to surface loudly.
- Preserve artifacts in CI when debugging by adding `--keep-artifacts` and uploading the temp dir. 
