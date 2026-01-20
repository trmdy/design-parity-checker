# Test fixture schema

Goal: deterministic paired screenshots + metadata for design discrepancy tests.

## Folder layout

```
test_assets/fixtures/<case_id>/
  ref.png
  impl.png
  meta.json

Multiple viewports:

```
test_assets/fixtures/<case_id>/
  ref.desktop.png
  impl.desktop.png
  ref.mobile.png
  impl.mobile.png
  meta.json
```

## meta.json schema (v1)

```
{
  "schema_version": "1.0",
  "case_id": "hero-typography-001",
  "title": "Hero H1 size + weight change",
  "category": ["typography"],
  "complexity": "low",
  "mutations": [
    {
      "target": "h1.hero-title",
      "property": "font-size",
      "from": "64px",
      "to": "56px",
      "delta": "small"
    },
    {
      "target": "h1.hero-title",
      "property": "font-weight",
      "from": "700",
      "to": "600",
      "delta": "small"
    }
  ],
  "expectations": [
    {
      "label": "typography.size",
      "severity": "low"
    },
    {
      "label": "typography.weight",
      "severity": "low"
    }
  ],
  "assertions": {
    "pixel_regions_min": 1,
    "similarity_max": 0.992,
    "typography_score_max": 0.999
  },
  "assertions_by_viewport": {
    "desktop": { "pixel_regions_min": 0, "similarity_max": 1.0 },
    "mobile": { "pixel_regions_min": 1, "similarity_max": 0.995 }
  },
  "viewport": { "width": 1280, "height": 720, "device_scale_factor": 1 },
  "viewports": [
    { "name": "desktop", "width": 1280, "height": 720, "device_scale_factor": 1 },
    { "name": "mobile", "width": 390, "height": 844, "device_scale_factor": 2 }
  ],
  "ignore_regions": [
    { "x": 0, "y": 0, "width": 120, "height": 40 }
  ],
  "ignore_regions_by_viewport": {
    "mobile": [{ "x": 0, "y": 0, "width": 120, "height": 40 }]
  },
  "assets": {
    "fonts": ["Space Grotesk 1.2.0"],
    "images": []
  },
  "notes": "Single change, safe crop-free."
}
```

Field notes:
- `category`: multi-tag. Use: `typography`, `layout`, `color`, `spacing`, `alignment`, `icon`, `image`, `copy`, `component`.
- `complexity`: `low|medium|high` (increasing visual scope).
- `delta`: `tiny|small|medium|large`.
- `assertions`: optional auto-checks for fixture validation.
- `assertions_by_viewport`: per-viewport assertion overrides.
- `expectations.label`: stable taxonomy for evaluator; add new labels as needed.
- `viewport`: single-viewport case.
- `viewports`: multi-viewport case; `name` must match filename suffix.
- `ignore_regions`: list of rects to mask before metrics (global).
- `ignore_regions_by_viewport`: per-viewport masks when needed.
- `assertions.*` supported keys: `pixel_regions_min`, `similarity_max`, `color_diffs_min`, `color_score_max`, `typography_score_max`, `layout_score_max`, `content_score_max`.

## Example cases

### 1) Color change (button)

```
{
  "schema_version": "1.0",
  "case_id": "cta-color-001",
  "title": "Primary button fill color shift",
  "category": ["color", "component"],
  "mutations": [
    {
      "target": "button.primary",
      "property": "background-color",
      "from": "#0E5A8A",
      "to": "#0E6E9A",
      "delta": "tiny"
    }
  ],
  "expectations": [
    { "label": "color.fill", "severity": "low" }
  ],
  "viewport": { "width": 1280, "height": 720, "device_scale_factor": 1 }
}
```

### 2) Layout change (grid)

```
{
  "schema_version": "1.0",
  "case_id": "features-grid-002",
  "title": "Grid columns 3 -> 2",
  "category": ["layout", "spacing"],
  "mutations": [
    {
      "target": ".features-grid",
      "property": "grid-template-columns",
      "from": "repeat(3, 1fr)",
      "to": "repeat(2, 1fr)",
      "delta": "large"
    }
  ],
  "expectations": [
    { "label": "layout.structure", "severity": "high" },
    { "label": "spacing.density", "severity": "medium" }
  ],
  "viewport": { "width": 1280, "height": 720, "device_scale_factor": 1 }
}
```

### 3) Spacing change (card)

```
{
  "schema_version": "1.0",
  "case_id": "card-padding-003",
  "title": "Card padding 24 -> 32",
  "category": ["spacing", "component"],
  "mutations": [
    {
      "target": ".card",
      "property": "padding",
      "from": "24px",
      "to": "32px",
      "delta": "small"
    }
  ],
  "expectations": [
    { "label": "spacing.padding", "severity": "medium" }
  ],
  "viewport": { "width": 1280, "height": 720, "device_scale_factor": 1 }
}
```

### 4) Multi-viewport + ignore mask

```
{
  "schema_version": "1.0",
  "case_id": "header-nav-004",
  "title": "Mobile nav icon mismatch + cookie mask",
  "category": ["icon", "layout"],
  "mutations": [
    {
      "target": ".nav-toggle svg",
      "property": "path",
      "from": "hamburger",
      "to": "kebab",
      "delta": "small"
    }
  ],
  "expectations": [
    { "label": "icon.mismatch", "severity": "medium" }
  ],
  "viewports": [
    { "name": "desktop", "width": 1280, "height": 720, "device_scale_factor": 1 },
    { "name": "mobile", "width": 390, "height": 844, "device_scale_factor": 2 }
  ],
  "ignore_regions_by_viewport": {
    "mobile": [{ "x": 0, "y": 760, "width": 390, "height": 84 }]
  }
}
```
