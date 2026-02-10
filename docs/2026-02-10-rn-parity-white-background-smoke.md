# RN Parity Smoke: White Background Bias (2026-02-10)

Context: `/tmp/rn-parity-full-core` legacy vs RN captures.

## Commands

- `target/debug/dpc compare --ref <legacy> --impl <rn> --format json`
- `target/debug/dpc compare --ref <legacy> --impl <rn> --metrics pixel --format json`

## Findings

- Legacy screenshots are near-blank white on most routes: `99.7%` white pixels (`/` route: `67.6%`).
- RN screenshots are also mostly white (`83%` to `92%` white; `rn_raw` often even higher).
- Pixel-only similarity remains high (`0.9157` to `0.9394`) despite visible UI differences.
- Full similarity (pixel+color) is lower (`0.8810` to `0.8976`) due color metric cap, but still passes prior gate threshold `0.82` on all routes.

## Per-route (`legacy -> rn`)

| route | legacy white% | rn white% | full sim | pixel-only sim |
|---|---:|---:|---:|---:|
| `/` | 67.6 | 75.5 | 0.8446 | 0.8638 |
| `checklists` | 99.7 | 83.1 | 0.8810 | 0.9157 |
| `customers` | 99.7 | 85.2 | 0.8824 | 0.9178 |
| `deviations` | 99.7 | 86.7 | 0.8864 | 0.9234 |
| `offers` | 99.7 | 87.0 | 0.8876 | 0.9251 |
| `projects` | 99.7 | 88.1 | 0.8883 | 0.9261 |
| `settings` | 99.7 | 91.6 | 0.8976 | 0.9394 |
| `timesheets` | 99.7 | 87.3 | 0.8890 | 0.9271 |

## Gate impact snapshot (`legacy -> rn`, full score)

- `>= 0.82`: `8/8`
- `>= 0.90`: `0/8`
- `min/max`: `0.8446 / 0.8976`

Conclusion: white-background dominance still inflates pixel metric for this corpus; full score partially masks it via color cap, but current gate threshold (`0.82`) remains too permissive.
