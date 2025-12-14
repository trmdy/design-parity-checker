# Configuration

DPC accepts a TOML config file (e.g., `dpc.toml`) via `--config <path>`. CLI flags always override config values.

## Supported keys
- `viewport`: either `"WIDTHxHEIGHT"` (e.g., `"1440x900"`) or a table `{ width = 1440, height = 900 }`
- `threshold`: `0.0`–`1.0`
- `[metric_weights]`: `pixel`, `layout`, `typography`, `color`, `content` (all must be > 0)
- `[timeouts]`: `navigation`, `network_idle`, `process` as human-friendly durations (`"30s"`, `"2m"`, etc.)

Invalid or missing values yield a config error (exit code 2) before any rendering. Use `--verbose` to log the effective config.

## Example
```toml
viewport = "1280x720"
threshold = 0.9

[metric_weights]
pixel = 0.4
layout = 0.2
typography = 0.15
color = 0.15
content = 0.1

[timeouts]
navigation = "20s"
network_idle = "8s"
process = "45s"
```
