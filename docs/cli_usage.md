# CLI Usage Cheatsheet

Commands:
- `dpc compare --ref <resource> --impl <resource> [--ref-type/--impl-type] [--viewport WxH] [--threshold FLOAT] [--metrics list] [--ignore-selectors ".ads,#banner"] [--ignore-regions regions.json] [--format json|pretty] [--output PATH] [--keep-artifacts|--artifacts-dir PATH]`
- `dpc generate-code --input <resource> [--stack html+tailwind] [--viewport WxH] [--output PATH] [--format json|pretty]` (stub)
- `dpc quality --input <resource> [--viewport WxH] [--output PATH] [--format json|pretty]` (stub)

Global flags:
- `--config <PATH>`: optional TOML to set defaults (viewport, threshold, metric weights, timeouts) when wiring is enabled.
- `--verbose`: prints basic progress.

Key options:
- `--viewport`: default `1440x900`.
- `--threshold`: default `0.95` for compare.
- `--metrics`: comma list of `pixel,layout,typography,color,content`; if omitted, all available metrics run (pixel+color when no DOM/figma).
- `--ignore-selectors`: comma-separated CSS selectors to drop DOM nodes before structural metrics.
- `--ignore-regions`: JSON array of `{x,y,width,height}` rectangles to mask before pixel/color metrics.
- `--keep-artifacts` or `--artifacts-dir`: retain screenshots/DOM/Figma exports; artifacts block surfaces in output.

Outputs:
- `--format json|pretty` (pretty currently prints JSON text).
- Exit codes: 0 pass / stub success; 1 threshold fail; 2 errors.

Resources:
- Auto-detected: url | image | figma; override with `--*-type`.
- Figma requires `FIGMA_TOKEN` and `node-id` in the URL.
- URL rendering requires Node + Playwright + Chromium download.

## Examples
- Image vs image (JSON):  
  `dpc compare --ref ref.png --impl impl.png --threshold 0.95 --format json`
- URL vs URL with mocks (no browser hit):  
  `DPC_MOCK_RENDER_REF=ref.png DPC_MOCK_RENDER_IMPL=impl.png dpc compare --ref https://design --impl https://build --format pretty`
- Figma vs image (needs FIGMA_TOKEN):  
  `FIGMA_TOKEN=... dpc compare --ref https://www.figma.com/file/FILE/Design?node-id=1-2 --impl impl.png --ref-type figma --format json --keep-artifacts`
- Ignore regions (mask pixel/color):  
  `dpc compare --ref ref.png --impl impl.png --ignore-regions regions.json --format json`  
  `regions.json` is an array of `{x,y,width,height}` (or `w,h`), values in px or 0–1 (percent of viewport). Example full-frame mask: `[{"x":0,"y":0,"w":1,"h":1}]`.
- Keep artifacts in custom dir:  
  `dpc compare --ref ref.png --impl impl.png --artifacts-dir artifacts/run1 --threshold 0.9 --format pretty`  
  Artifacts (screenshots/DOM/Figma) are retained under the chosen directory.
- Generate code (stub output):  
  `dpc generate-code --input https://www.figma.com/file/FILE/Design?node-id=1-2 --stack html+tailwind --format json`
- Quality (experimental stub):  
  `dpc quality --input impl.png --format pretty`
