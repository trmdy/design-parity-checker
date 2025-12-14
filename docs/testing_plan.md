# Testing Plan

Practical steps to validate the CLI today, with and without external deps.

## Foundational
- `cargo fmt --all --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test -- --nocapture`
- With OCR feature: `cargo test --features ocr -- --nocapture`

## Core CLI flows (no browser/Figma; use bundled fixtures/mocks)
1) Image parity (JSON)  
   `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --format json`
2) Pretty TTY vs piped/file JSON  
   - TTY: `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --format pretty`  
   - Piped/file: `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --format pretty --output out.json`
3) Threshold fail exit code  
   `dpc compare --ref test_assets/ref.png --impl test_assets/big_diff.png --threshold 0.99`
4) Ignore selectors (DOM filter)  
   `dpc compare --ref test_assets/ref_dom.png --impl test_assets/impl_dom.png --ignore-selectors ".ads,#cookie"`
5) Ignore regions (pixel mask)  
   `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --ignore-regions test_assets/ignore_regions_example.json`
6) Keep artifacts  
   `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --keep-artifacts --artifacts-dir artifacts/run1`
7) Config file + flag precedence (use --verbose to see effective config)  
   `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --config dpc.toml --process-timeout 50 --format json --verbose`

## Codegen + quality quick checks
- Generate code via mock backend: `DPC_MOCK_CODE="<main>demo</main>" dpc generate-code --input test_assets/ref.png --format pretty --output demo.html`
- Quality heuristics (JSON): `dpc quality --input test_assets/ref.png --format json`
- Quality heuristics (pretty): `dpc quality --input test_assets/ref.png --format pretty`

## Quality heuristics validation
The quality command now includes contrast and hierarchy analysis:
1) Hierarchy detection  
   - Create test image with mixed font sizes → expect `missing_hierarchy` finding with tier count
   - Single font size → expect warning about insufficient hierarchy
   - 2-3 distinct sizes → expect info-level healthy hierarchy message
2) Contrast heuristic  
   - Low contrast text (light gray on white) → expect `low_contrast` warning
   - High contrast text (dark on white) → expect info-level pass
   - Verify contrast ratio calculation matches WCAG formula
3) Combined scoring  
   `dpc quality --input test_assets/ref.png --format json | jq '.score, .findings'`

## Network/browser (Playwright/Node required)
Prereq: `npm install playwright && npx playwright install chromium`
- URL vs URL: `dpc compare --ref https://example.com --impl https://example.org --format json --verbose`
- Timeout behavior: `dpc compare --ref https://example.com --impl https://example.org --nav-timeout 5 --network-idle-timeout 3`

## Figma (token + node-id required)
Prereq: export `FIGMA_TOKEN`
- `dpc compare --ref "https://www.figma.com/file/<KEY>/Design?node-id=1-2" --impl test_assets/impl.png --ref-type figma --format json --keep-artifacts`
- Negative cases: missing token or node-id → expect remediation in output.

## Mocked Figma/URL (no network)
- `DPC_MOCK_RENDER_REF=/path/ref.png DPC_MOCK_RENDER_IMPL=/path/impl.png dpc compare --ref https://ref --impl https://impl --format pretty`
- `DPC_MOCK_RENDERERS_DIR=/path/dir_with_ref_impl dpc compare --ref https://ref --impl https://impl --format json`

## Artifact discoverability
Artifacts are now always surfaced in output (even when not retained):
1) Default run (no --keep-artifacts)  
   `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --format json`
   - Expect: `artifacts.kept = false`, directory path shown, hint about --keep-artifacts
   - Stderr should show "Artifacts directory: ... (cleaned up after run)"
2) With --keep-artifacts  
   `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --keep-artifacts`
   - Expect: `artifacts.kept = true`, all paths valid, hint about viewing diff heatmap
3) Pretty output artifact hints  
   - Verify hints appear: "open the diff heatmap or DOM snapshots" or "rerun with --keep-artifacts"

## OCR feature (optional, requires Tesseract)
Prereq: Install Tesseract (`brew install tesseract` or `apt install tesseract-ocr`)
Build: `cargo build --features ocr`

1) OCR availability check  
   - With feature: `dpc_lib::ocr_is_available()` returns `true`
   - Without feature: returns `false`, OCR gracefully skipped
2) Image-to-image with OCR content extraction  
   `cargo run --features ocr -- compare --ref test_assets/ref.png --impl test_assets/impl.png --format json`
   - Check if `ocr_blocks` populated in debug output (requires text in images)
3) OCR error handling  
   - Missing tessdata → expect graceful fallback, not crash
   - Invalid image path → expect clear error message

## Hierarchy metric (compare mode)
The compare command now includes a hierarchy metric for typography analysis:
1) Verify hierarchy metric in scores  
   `dpc compare --ref test_assets/ref.png --impl test_assets/impl.png --format json | jq '.metrics.hierarchy'`
   - Expect: `score`, `tier_count`, `distinct_tiers`, `issues` fields
2) DOM/Figma hierarchy detection  
   - Views with 2-5 distinct font size tiers → score = 1.0
   - Views with <2 or >5 tiers → score < 1.0, issues populated

## Regression focus
- Pretty vs JSON parity: `--format pretty --output file` still JSON.
- Artifacts block present in all compare outputs (kept field indicates retention).
- Error remediation text: Playwright install, FIGMA_TOKEN/node-id, unsupported extension, timeouts.
- Metric result structures: all metrics now use `issues` arrays instead of flat fields.

## What you need
- Rust toolchain (already used by `cargo test`).
- For browser runs: Node + Playwright + Chromium download.
- For Figma runs: `FIGMA_TOKEN` and a URL with `node-id`.
- For OCR: Tesseract installed, build with `--features ocr`.
- You can start today with the mock-based suite + full `cargo test`; add browser/Figma/OCR runs when deps/tokens are available.

## New metric structures checklist
After recent refactoring, verify these metric result shapes:

| Metric | Key fields | Issue enum |
|--------|-----------|------------|
| `pixel` | `score`, `diff_regions[]` | `PixelDiffRegion` with severity/reason |
| `layout` | `score`, `issues[]` | `LayoutIssue::MissingElement/ExtraElement/PositionShift/SizeChange` |
| `typography` | `score`, `issues[]` | `TypographyIssue::FontFamilyMismatch/FontSizeDiff/...` |
| `color` | `score`, `issues[]` | `ColorIssue::PrimaryColorShift/AccentColorShift/...` |
| `content` | `score`, `issues[]` | `ContentIssue::MissingText/ExtraText` |
| `hierarchy` | `score`, `tier_count`, `distinct_tiers[]`, `issues[]` | `HierarchyIssue::TooFewTiers/TooManyTiers` |

Verify with: `dpc compare ... --format json | jq '.metrics'`
