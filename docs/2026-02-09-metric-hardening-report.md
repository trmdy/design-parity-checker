# DPC Metric Hardening Report (2026-02-09)

## Problem statement

Current similarity can be falsely high when:

1. Large shared backgrounds dominate pixel-average scoring.
2. Only small but critical UI regions differ (header, CTA, nav, badges).
3. One side is a wrong route/error page (e.g. 404), yet score still appears acceptable in some scenarios.

Observed in real parity runs where route-mapping mistakes and low-information backgrounds masked important UI mismatch.

## Root causes

- Global mean absolute pixel metric (`1 - mean(abs(diff))`) is unweighted.
- No semantic region weighting.
- No automatic low-information/background discounting.
- No route/error-state sentinel checks in image-only mode.

## Improvement goals

1. Keep current metric as a baseline for speed/backward compatibility.
2. Add robust perceptual + structural metrics.
3. Add region-aware scoring with explicit critical-zone weighting.
4. Add guardrails to fail fast on obviously invalid comparisons.

## Proposed scoring pipeline

Final score:

`score = w1*ssim + w2*lpips_like + w3*edge_iou + w4*color_hist + w5*pixel_baseline`

Suggested initial weights:

- `ssim`: 0.30
- `lpips_like` (or CLIP-embedding distance fallback): 0.25
- `edge_iou`: 0.20
- `color_hist`: 0.10
- `pixel_baseline`: 0.15

### Region weighting layer

Compute per-region scores, then weighted aggregate:

- `header`: 0.20
- `primary content`: 0.40
- `primary CTA(s)`: 0.20
- `navigation/footer`: 0.20

Region detection modes:

1. `auto`: detect via OCR + edge/contour + recurring UI anchors.
2. `grid`: deterministic NxM segmentation (fallback).
3. `manual`: user-provided ROI boxes.

## Background suppression (with caveats)

Goal: reduce background dominance, not remove all background signal.

Method:

1. Build low-information mask from:
   - low local variance
   - near-uniform color spans
   - weak edge density
2. Cap contribution of masked pixels (not full exclusion).
3. Preserve text/icon/edge islands within background zones.

Caveats:

- Never fully ignore background by default.
- Keep configurable cap (`--bg-max-weight`, default 0.35 of total score budget).
- Expose mask artifact so users can audit what was discounted.

## Invalid-compare sentinels

Before scoring:

1. OCR pass for known error patterns:
   - `404`, `Not found`, `Unmatched Route`, `Render Error`, `Tenant not found`.
2. Detect debug overlay noise (error stack overlays, dev banners).
3. If one side has error sentinel and the other does not:
   - return `invalid_compare=true`
   - force low confidence and hard fail unless `--allow-invalid-compare`.

## New CLI/API proposals

- `--metric-profile parity-v2` (new default after stabilization)
- `--roi auto|grid|manual`
- `--roi-box x,y,w,h` (repeatable)
- `--bg-suppression on|off`
- `--bg-max-weight <0..1>`
- `--critical-region header,cta,nav,...` (repeatable, boosts penalties)
- `--detect-error-sentinels on|off`
- `--invalid-compare-policy fail|warn|ignore`
- JSON output:
  - `invalid_compare`
  - `error_sentinels`
  - `regions[]` with per-region scores
  - `background_mask_ratio`

## Artifacts to add

- `region_map.png`
- `background_mask.png`
- `sentinel_ocr.json`
- `score_breakdown.json`

## Test plan

Add fixtures for:

1. Same screen, minor spacing/icon diffs.
2. Same screen, major CTA/header mismatch.
3. Correct screen vs error page.
4. Correct screen vs blank/near-blank page.
5. Same structure, different theme tokens.
6. Mobile viewport with safe-area/status bar variance.

Assertions:

- Error-page comparisons flagged `invalid_compare=true`.
- Region-critical mismatches reduce score significantly even with similar backgrounds.
- Background-only changes have bounded influence.

## Rollout plan

1. Ship `parity-v2` behind opt-in profile.
2. Collect score deltas on existing benchmark corpus.
3. Tune weights/thresholds.
4. Flip default profile once regression suite passes and score stability is acceptable.

## Success criteria

- Fewer false-high scores on mismatched routes/error pages.
- Better correlation with human parity judgment.
- Clear actionable artifacts for developers and agents.
