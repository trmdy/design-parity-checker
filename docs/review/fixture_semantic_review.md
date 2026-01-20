# Review: Fixture corpus + semantic runner

## Findings

### Medium
- Coverage gap: many low‑delta typography cases no longer assert semantic detection because `default_assertions` only gates `typography_score_max` on medium/large deltas. This means most `typography` cases can pass even if the typography metric never flags them. Consider keeping a weaker threshold for small deltas or adding explicit per‑case assertions (e.g., `typography_score_max: 0.999`). (`test_assets/generate_fixture_sources.py` lines 496–509)

### Low
- Schema drift: `assertions_by_viewport` and new assertion keys (`color_score_max`, `typography_score_max`, `content_score_max`) are not documented in the fixture schema doc, so readers won’t know they exist. (`docs/test_fixture_schema.md` lines 24–92)
- `run_fixture_checks.py` iterates cases from `test_assets/fixtures/` but reads HTML from `test_assets/fixtures_src/`. If only sources exist, the run silently does zero cases; consider warning when `fixtures_dir` is empty or HTML files are missing. (`test_assets/run_fixture_checks.py` lines 225–285)

## Questions / Assumptions
- Should we commit generated assets (`test_assets/fixtures/`, `test_assets/fixtures_src/`) to the repo or keep them locally? Current git status shows both untracked.
- Do we want small‑delta typography cases to be strictly asserted, or is “medium/large only” acceptable coverage for now?

## Summary
Strong: Playwright arg fix, semantic runner with per‑viewport reporting, and grid reorder deltas now validated. Main risk is coverage loss for small typography diffs and schema doc drift.
