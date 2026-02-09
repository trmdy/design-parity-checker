# Background-Dominance Penalty (2026-02-09)

## Problem

`PixelSimilarity` used global SSIM only.
Result: high score even when meaningful foreground region changed, if most background stayed identical.

## Change

File: `src/metrics/pixel.rs`

Added coverage penalty on top of SSIM:

- Build luma diff map (already present).
- Compute changed-pixel ratio where `diff >= 0.02`.
- Penalty = `min(0.25 * sqrt(changed_ratio), 0.30)`.
- Final pixel score = `clamp(ssim - penalty, 0.0, 1.0)`.

Intent: suppress false-high scores from background dominance while keeping tiny localized diffs near-high.

## Regression tests

File: `src/metrics/tests.rs`

Added:

- `pixel_metric_penalizes_large_localized_difference_on_shared_background`
- `pixel_metric_keeps_tiny_localized_difference_high`

## Fixture impact

Command:

`python3 -u test_assets/run_fixture_checks.py --cmd "cargo run --quiet --" --strict`

Before patch: `Failures: 7`
After patch: `Failures: 5`

Resolved false-high cases:

- `t3-grid-reorder-001`
- `t3-grid-reorder-004`

Remaining failures are unrelated to this penalty:

- `t1-hero-subtitle-color-001` (tight color threshold)
- `t3-*-multi-viewport-*` desktop-only compare path producing exact 1.0 while mutation exists in mobile viewport assertions.
