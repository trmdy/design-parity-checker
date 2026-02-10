# Release / Homebrew

Trigger: push a git tag `vX.Y.Z` (example: `v0.1.0`).

GitHub Actions: `.github/workflows/release.yml`

Outputs:
- GitHub Release assets: `dpc-x86_64-unknown-linux-gnu.tar.gz`, `dpc-x86_64-apple-darwin.tar.gz`, `dpc-aarch64-apple-darwin.tar.gz`, `dpc-x86_64-pc-windows-msvc.zip`

Homebrew (optional):
- Requires secret: `HOMEBREW_TAP_GITHUB_TOKEN` (PAT with push to tap repo).
- Optional repo variables:
- `HOMEBREW_TAP_REPO` (default: `trmdy/homebrew-tap`)
- `HOMEBREW_FORMULA_PATH` (default: `Formula/dpc.rb`)

Install:
- `brew install trmdy/tap/dpc`
