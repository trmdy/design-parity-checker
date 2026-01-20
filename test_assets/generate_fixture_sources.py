#!/usr/bin/env python3
import json
from pathlib import Path
from textwrap import dedent


def hero_template(**overrides):
    cfg = {
        "bg": "#f6f4ef",
        "ink": "#141414",
        "accent": "#c66a2b",
        "title_size": "64px",
        "title_weight": "700",
        "title_letter_spacing": "-0.5px",
        "title_margin": "0 0 16px 0",
        "title_line_height": "1.05",
        "subtitle_size": "20px",
        "subtitle_color": "#3a3a3a",
        "pill_letter_spacing": "1px",
        "button_padding": "12px 20px",
        "button_radius": "999px",
        "columns": "1.2fr 0.8fr",
        "gap": "48px",
        "panel_border": "1px solid #e3ddd2",
        "panel_radius": "16px",
        "panel_shadow": "0 20px 40px rgba(0,0,0,0.08)",
        "panel_display": "block",
    }
    cfg.update(overrides)
    return dedent(
        f"""
        <!doctype html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1">
          <title>Hero</title>
          <style>
            :root {{
              --bg: {cfg["bg"]};
              --ink: {cfg["ink"]};
              --accent: {cfg["accent"]};
            }}
            * {{ box-sizing: border-box; }}
            body {{
              margin: 0;
              font-family: "Helvetica Neue", Arial, sans-serif;
              background: var(--bg);
              color: var(--ink);
            }}
            .frame {{
              width: 100%;
              min-height: 100vh;
              display: flex;
              align-items: center;
              justify-content: center;
              padding: 64px;
            }}
            .hero {{
              width: 100%;
              max-width: 1040px;
              display: grid;
              grid-template-columns: {cfg["columns"]};
              gap: {cfg["gap"]};
              align-items: center;
            }}
            .hero-title {{
              font-size: {cfg["title_size"]};
              line-height: {cfg["title_line_height"]};
              font-weight: {cfg["title_weight"]};
              margin: {cfg["title_margin"]};
              letter-spacing: {cfg["title_letter_spacing"]};
            }}
            .hero-subtitle {{
              font-size: {cfg["subtitle_size"]};
              line-height: 1.5;
              margin: 0 0 24px 0;
              color: {cfg["subtitle_color"]};
            }}
            .hero-pill {{
              display: inline-block;
              padding: 6px 12px;
              border-radius: 999px;
              background: var(--accent);
              color: white;
              font-size: 12px;
              letter-spacing: {cfg["pill_letter_spacing"]};
              text-transform: uppercase;
            }}
            .hero-cta {{
              display: inline-flex;
              align-items: center;
              padding: {cfg["button_padding"]};
              border-radius: {cfg["button_radius"]};
              background: var(--ink);
              color: white;
              border: none;
              font-weight: 600;
              letter-spacing: 0.3px;
            }}
            .panel {{
              display: {cfg["panel_display"]};
              padding: 28px;
              border-radius: {cfg["panel_radius"]};
              border: {cfg["panel_border"]};
              box-shadow: {cfg["panel_shadow"]};
              background: white;
            }}
            .panel-stat {{
              font-size: 40px;
              font-weight: 700;
              margin-bottom: 8px;
            }}
            .panel-label {{
              color: #6b5f55;
              text-transform: uppercase;
              font-size: 12px;
              letter-spacing: 1.2px;
            }}
          </style>
        </head>
        <body>
          <div class="frame">
            <section class="hero">
              <div>
                <span class="hero-pill">Spring Release</span>
                <h1 class="hero-title">Design parity, faster.</h1>
                <p class="hero-subtitle">
                  Compare pixel differences and find issues in seconds, not hours.
                </p>
                <button class="hero-cta">Start a review</button>
              </div>
              <aside class="panel">
                <div class="panel-stat">98%</div>
                <div class="panel-label">Match accuracy</div>
              </aside>
            </section>
          </div>
        </body>
        </html>
        """
    ).strip() + "\n"


def card_template(**overrides):
    cfg = {
        "bg": "#f4f7fb",
        "card_bg": "#ffffff",
        "border": "1px solid #d6dbe5",
        "radius": "22px",
        "padding": "32px",
        "shadow": "0 18px 40px rgba(15,23,42,0.12)",
        "title_size": "28px",
        "body_size": "16px",
        "badge_letter_spacing": "2px",
        "badge_bg": "#e2e8f0",
        "button_padding": "10px 16px",
    }
    cfg.update(overrides)
    return dedent(
        f"""
        <!doctype html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1">
          <title>Card</title>
          <style>
            * {{ box-sizing: border-box; }}
            body {{
              margin: 0;
              font-family: "Helvetica Neue", Arial, sans-serif;
              background: {cfg["bg"]};
              color: #0f172a;
            }}
            .frame {{
              min-height: 100vh;
              display: grid;
              place-items: center;
              padding: 56px;
            }}
            .card {{
              width: 520px;
              background: {cfg["card_bg"]};
              border: {cfg["border"]};
              border-radius: {cfg["radius"]};
              padding: {cfg["padding"]};
              box-shadow: {cfg["shadow"]};
            }}
            h2 {{
              margin: 0 0 12px 0;
              font-size: {cfg["title_size"]};
              letter-spacing: -0.3px;
            }}
            p {{
              margin: 0 0 20px 0;
              font-size: {cfg["body_size"]};
              color: #475569;
            }}
            .badge {{
              display: inline-block;
              padding: 6px 12px;
              border-radius: 999px;
              background: {cfg["badge_bg"]};
              font-size: 11px;
              letter-spacing: {cfg["badge_letter_spacing"]};
              text-transform: uppercase;
            }}
            .action {{
              display: inline-flex;
              align-items: center;
              padding: {cfg["button_padding"]};
              border-radius: 10px;
              border: 1px solid #0f172a;
              background: white;
              font-weight: 600;
              font-size: 13px;
            }}
          </style>
        </head>
        <body>
          <div class="frame">
            <div class="card">
              <span class="badge">Diff Review</span>
              <h2>Sync issue backlog</h2>
              <p>Queue visual mismatches and resolve the top offenders first.</p>
              <button class="action">View queue</button>
            </div>
          </div>
        </body>
        </html>
        """
    ).strip() + "\n"


def grid_template(**overrides):
    cfg = {
        "bg": "#0b0f1a",
        "columns": "repeat(3, 1fr)",
        "gap": "20px",
        "grid_areas": None,
        "tile_areas": None,
        "tile_bg": "#111827",
        "tile_border": "1px solid #1f2937",
        "tile_radius": "18px",
        "title": "Feature Grid",
        "order": [0, 1, 2, 3, 4, 5],
        "tile_count": 6,
    }
    cfg.update(overrides)
    tiles = []
    for idx in range(cfg["tile_count"]):
        order = cfg["order"][idx] if idx < len(cfg["order"]) else idx
        area = ""
        if cfg["tile_areas"] and idx < len(cfg["tile_areas"]):
            area = f' grid-area: {cfg["tile_areas"][idx]};'
        tiles.append(
            f'<div class="tile" style="order:{order};{area}">'
            f"<h3>Tile {idx + 1}</h3>"
            "<p>Pixel checks, tokens, and layout hints.</p>"
            "</div>"
        )
    tiles_html = "\n      ".join(tiles)
    grid_areas = ""
    if cfg["grid_areas"]:
        grid_areas = f"grid-template-areas: {cfg['grid_areas']};"
    return dedent(
        f"""
        <!doctype html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1">
          <title>{cfg["title"]}</title>
          <style>
            * {{ box-sizing: border-box; }}
            body {{
              margin: 0;
              font-family: "Helvetica Neue", Arial, sans-serif;
              background: {cfg["bg"]};
              color: #f8fafc;
            }}
            .frame {{
              min-height: 100vh;
              padding: 56px;
            }}
            h1 {{
              margin: 0 0 24px 0;
              font-size: 32px;
            }}
            .grid {{
              display: grid;
              grid-template-columns: {cfg["columns"]};
              gap: {cfg["gap"]};
              {grid_areas}
            }}
            .tile {{
              background: {cfg["tile_bg"]};
              border: {cfg["tile_border"]};
              border-radius: {cfg["tile_radius"]};
              padding: 20px;
            }}
            .tile h3 {{
              margin: 0 0 8px 0;
              font-size: 18px;
            }}
            .tile p {{
              margin: 0;
              color: #cbd5f5;
            }}
          </style>
        </head>
        <body>
          <div class="frame">
            <h1>{cfg["title"]}</h1>
            <div class="grid">
              {tiles_html}
            </div>
          </div>
        </body>
        </html>
        """
    ).strip() + "\n"


def stats_template(**overrides):
    cfg = {
        "bg": "#f8fafc",
        "align": "center",
        "title_size": "30px",
        "aside_border": "1px solid #e2e8f0",
        "aside_bg": "#ffffff",
    }
    cfg.update(overrides)
    return dedent(
        f"""
        <!doctype html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1">
          <title>Stats</title>
          <style>
            * {{ box-sizing: border-box; }}
            body {{
              margin: 0;
              font-family: "Helvetica Neue", Arial, sans-serif;
              background: {cfg["bg"]};
              color: #0f172a;
            }}
            .frame {{
              min-height: 100vh;
              display: flex;
              align-items: center;
              justify-content: center;
              padding: 64px;
            }}
            .layout {{
              display: flex;
              gap: 48px;
              align-items: {cfg["align"]};
            }}
            .stats {{
              display: grid;
              gap: 16px;
            }}
            .stat {{
              padding: 16px 20px;
              border-radius: 14px;
              background: white;
              border: 1px solid #e2e8f0;
            }}
            .stat strong {{
              display: block;
              font-size: {cfg["title_size"]};
              margin-bottom: 4px;
            }}
            .aside {{
              padding: 28px;
              border-radius: 16px;
              border: {cfg["aside_border"]};
              background: {cfg["aside_bg"]};
              max-width: 260px;
            }}
          </style>
        </head>
        <body>
          <div class="frame">
            <div class="layout">
              <div class="stats">
                <div class="stat"><strong>1.2k</strong>comparisons</div>
                <div class="stat"><strong>92%</strong>pass rate</div>
                <div class="stat"><strong>48</strong>open tasks</div>
              </div>
              <div class="aside">
                <h3>Weekly focus</h3>
                <p>Reduce typography drift and spacing regressions.</p>
              </div>
            </div>
          </div>
        </body>
        </html>
        """
    ).strip() + "\n"


def nav_template(**overrides):
    cfg = {
        "bg": "#0f172a",
        "accent": "#38bdf8",
        "icon_path": "M3 6h18M3 12h18M3 18h18",
        "mobile_icon_path": None,
    }
    cfg.update(overrides)
    mobile_icon = cfg["mobile_icon_path"] or cfg["icon_path"]
    return dedent(
        f"""
        <!doctype html>
        <html lang="en">
        <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1">
          <title>Nav</title>
          <style>
            * {{ box-sizing: border-box; }}
            body {{
              margin: 0;
              font-family: "Helvetica Neue", Arial, sans-serif;
              background: {cfg["bg"]};
              color: #f8fafc;
            }}
            header {{
              display: flex;
              align-items: center;
              justify-content: space-between;
              padding: 24px 40px;
            }}
            .brand {{
              font-weight: 700;
              letter-spacing: 0.5px;
            }}
            nav {{
              display: flex;
              gap: 24px;
            }}
            .cta {{
              padding: 10px 16px;
              border-radius: 999px;
              background: {cfg["accent"]};
              color: #0f172a;
              font-weight: 700;
            }}
            .mobile-toggle {{
              display: none;
            }}
            @media (max-width: 720px) {{
              nav {{ display: none; }}
              .mobile-toggle {{ display: block; }}
            }}
          </style>
        </head>
        <body>
          <header>
            <div class="brand">DPC</div>
            <nav>
              <span>Docs</span>
              <span>Pricing</span>
              <span>Cases</span>
              <span class="cta">Launch</span>
            </nav>
            <svg class="mobile-toggle" width="28" height="28" viewBox="0 0 24 24"
              fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
              <path class="desktop-path" d="{cfg["icon_path"]}"></path>
              <path class="mobile-path" d="{mobile_icon}"></path>
            </svg>
          </header>
        </body>
        </html>
        """
    ).strip() + "\n"


def case_meta(**kwargs):
    base = {
        "schema_version": "1.0",
        "assets": {"fonts": ["Helvetica Neue"], "images": []},
    }
    base.update(kwargs)
    return base


DELTA_ORDER = {"tiny": 0, "small": 1, "medium": 2, "large": 3}


def max_delta_level(mutations):
    if not mutations:
        return None
    levels = [DELTA_ORDER.get(m.get("delta")) for m in mutations]
    levels = [lvl for lvl in levels if lvl is not None]
    return max(levels) if levels else None


def has_typography_change(mutations):
    if not mutations:
        return False
    typography_props = {
        "font-size",
        "font-weight",
        "line-height",
        "letter-spacing",
        "font-family",
    }
    return any(m.get("property") in typography_props for m in mutations)


def default_assertions(category, complexity, mutations=None):
    assertions = {}
    if "layout" in category or "spacing" in category or "icon" in category:
        assertions["pixel_regions_min"] = 1
    if "color" in category:
        assertions["color_diffs_min"] = 1
        assertions["color_score_max"] = 0.9995
    if "typography" in category and has_typography_change(mutations):
        delta_level = max_delta_level(mutations)
        if delta_level == 0:
            assertions["typography_score_max"] = 0.9999
        elif delta_level == 1:
            assertions["typography_score_max"] = 0.999
        elif delta_level == 2:
            assertions["typography_score_max"] = 0.995
        elif delta_level == 3:
            assertions["typography_score_max"] = 0.99
    if "copy" in category:
        assertions["content_score_max"] = 0.95
    if complexity == "high":
        assertions["similarity_max"] = 0.995
    return assertions


def add_case(
    cases,
    case_id,
    title,
    category,
    complexity,
    mutations,
    expectations,
    ref_html,
    impl_html,
    viewport=None,
    viewports=None,
    notes=None,
    assertions=None,
    assertions_by_viewport=None,
):
    case = {
        "case_id": case_id,
        "title": title,
        "category": category,
        "complexity": complexity,
        "mutations": mutations,
        "expectations": expectations,
        "assertions": assertions or default_assertions(category, complexity, mutations),
        "ref_html": ref_html,
        "impl_html": impl_html,
    }
    if viewport:
        case["viewport"] = viewport
    if viewports:
        case["viewports"] = viewports
    if notes:
        case["notes"] = notes
    if assertions_by_viewport:
        case["assertions_by_viewport"] = assertions_by_viewport
    cases.append(case)


def main():
    out_dir = Path("test_assets/fixtures_src")
    out_dir.mkdir(parents=True, exist_ok=True)

    base_vp = {"width": 1280, "height": 720, "device_scale_factor": 1}
    viewports = [
        {"name": "desktop", "width": 1280, "height": 720, "device_scale_factor": 1},
        {"name": "mobile", "width": 390, "height": 844, "device_scale_factor": 2},
    ]

    cases = []

    hero_ref = hero_template()
    card_ref = card_template()
    grid_ref = grid_template()
    stats_ref = stats_template()
    nav_ref = nav_template()

    # Low complexity: single property deltas.
    hero_title_sizes = [
        ("62px", "tiny"),
        ("60px", "small"),
        ("58px", "small"),
        ("56px", "small"),
        ("54px", "medium"),
        ("52px", "medium"),
    ]
    for idx, (size, delta) in enumerate(hero_title_sizes, 1):
        add_case(
            cases,
            f"t1-hero-title-size-{idx:03d}",
            f"Hero title size to {size}",
            ["typography"],
            "low",
            [
                {
                    "target": ".hero-title",
                    "property": "font-size",
                    "from": "64px",
                    "to": size,
                    "delta": delta,
                }
            ],
            [{"label": "typography.size", "severity": "low"}],
            hero_ref,
            hero_template(title_size=size),
            viewport=base_vp,
        )

    hero_title_weights = [("600", "small"), ("500", "medium"), ("800", "small")]
    for idx, (weight, delta) in enumerate(hero_title_weights, 1):
        add_case(
            cases,
            f"t1-hero-title-weight-{idx:03d}",
            f"Hero title weight to {weight}",
            ["typography"],
            "low",
            [
                {
                    "target": ".hero-title",
                    "property": "font-weight",
                    "from": "700",
                    "to": weight,
                    "delta": delta,
                }
            ],
            [{"label": "typography.weight", "severity": "low"}],
            hero_ref,
            hero_template(title_weight=weight),
            viewport=base_vp,
        )

    subtitle_colors = ["#2d2d2d", "#4b4b4b", "#5a5a5a"]
    for idx, color in enumerate(subtitle_colors, 1):
        add_case(
            cases,
            f"t1-hero-subtitle-color-{idx:03d}",
            "Hero subtitle color shift",
            ["color", "typography"],
            "low",
            [
                {
                    "target": ".hero-subtitle",
                    "property": "color",
                    "from": "#3a3a3a",
                    "to": color,
                    "delta": "tiny",
                }
            ],
            [{"label": "color.text", "severity": "low"}],
            hero_ref,
            hero_template(subtitle_color=color),
            viewport=base_vp,
        )

    button_paddings = ["10px 18px", "12px 24px", "14px 20px", "14px 26px"]
    for idx, pad in enumerate(button_paddings, 1):
        add_case(
            cases,
            f"t1-hero-cta-padding-{idx:03d}",
            "Hero CTA padding change",
            ["spacing", "component"],
            "low",
            [
                {
                    "target": "button.hero-cta",
                    "property": "padding",
                    "from": "12px 20px",
                    "to": pad,
                    "delta": "small",
                }
            ],
            [{"label": "spacing.padding", "severity": "medium"}],
            hero_ref,
            hero_template(button_padding=pad),
            viewport=base_vp,
        )

    pill_letter_spacings = ["0.5px", "1.5px", "2px"]
    for idx, spacing in enumerate(pill_letter_spacings, 1):
        add_case(
            cases,
            f"t1-hero-pill-spacing-{idx:03d}",
            "Hero pill letter spacing",
            ["typography"],
            "low",
            [
                {
                    "target": ".hero-pill",
                    "property": "letter-spacing",
                    "from": "1px",
                    "to": spacing,
                    "delta": "tiny",
                }
            ],
            [{"label": "typography.letter_spacing", "severity": "low"}],
            hero_ref,
            hero_template(pill_letter_spacing=spacing),
            viewport=base_vp,
        )

    card_border_colors = ["#c7cdd8", "#bfc6d3", "#aeb7c6", "#d1d7e2"]
    for idx, color in enumerate(card_border_colors, 1):
        add_case(
            cases,
            f"t1-card-border-color-{idx:03d}",
            "Card border color change",
            ["color", "component"],
            "low",
            [
                {
                    "target": ".card",
                    "property": "border-color",
                    "from": "#d6dbe5",
                    "to": color,
                    "delta": "tiny",
                }
            ],
            [{"label": "color.stroke", "severity": "low"}],
            card_ref,
            card_template(border=f"1px solid {color}"),
            viewport=base_vp,
        )

    card_radii = ["18px", "16px", "14px", "26px"]
    for idx, radius in enumerate(card_radii, 1):
        add_case(
            cases,
            f"t1-card-radius-{idx:03d}",
            "Card radius change",
            ["shape", "component"],
            "low",
            [
                {
                    "target": ".card",
                    "property": "border-radius",
                    "from": "22px",
                    "to": radius,
                    "delta": "small",
                }
            ],
            [{"label": "shape.radius", "severity": "medium"}],
            card_ref,
            card_template(radius=radius),
            viewport=base_vp,
        )

    card_paddings = ["28px", "30px", "36px", "40px"]
    for idx, padding in enumerate(card_paddings, 1):
        add_case(
            cases,
            f"t1-card-padding-{idx:03d}",
            "Card padding change",
            ["spacing", "component"],
            "low",
            [
                {
                    "target": ".card",
                    "property": "padding",
                    "from": "32px",
                    "to": padding,
                    "delta": "small",
                }
            ],
            [{"label": "spacing.padding", "severity": "medium"}],
            card_ref,
            card_template(padding=padding),
            viewport=base_vp,
        )

    body_sizes = ["15px", "17px", "18px", "14px"]
    for idx, size in enumerate(body_sizes, 1):
        add_case(
            cases,
            f"t1-card-body-size-{idx:03d}",
            "Card body text size change",
            ["typography"],
            "low",
            [
                {
                    "target": ".card p",
                    "property": "font-size",
                    "from": "16px",
                    "to": size,
                    "delta": "tiny",
                }
            ],
            [{"label": "typography.size", "severity": "low"}],
            card_ref,
            card_template(body_size=size),
            viewport=base_vp,
        )

    badge_spacings = ["1px", "1.5px", "2.5px"]
    for idx, spacing in enumerate(badge_spacings, 1):
        add_case(
            cases,
            f"t1-card-badge-spacing-{idx:03d}",
            "Badge letter spacing change",
            ["typography"],
            "low",
            [
                {
                    "target": ".badge",
                    "property": "letter-spacing",
                    "from": "2px",
                    "to": spacing,
                    "delta": "tiny",
                }
            ],
            [{"label": "typography.letter_spacing", "severity": "low"}],
            card_ref,
            card_template(badge_letter_spacing=spacing),
            viewport=base_vp,
        )

    stats_title_sizes = ["28px", "26px", "32px"]
    for idx, size in enumerate(stats_title_sizes, 1):
        add_case(
            cases,
            f"t1-stats-title-size-{idx:03d}",
            "Stats title size change",
            ["typography"],
            "low",
            [
                {
                    "target": ".stat strong",
                    "property": "font-size",
                    "from": "30px",
                    "to": size,
                    "delta": "small",
                }
            ],
            [{"label": "typography.size", "severity": "low"}],
            stats_ref,
            stats_template(title_size=size),
            viewport=base_vp,
        )

    nav_accents = ["#7dd3fc", "#22d3ee", "#fbbf24"]
    for idx, color in enumerate(nav_accents, 1):
        add_case(
            cases,
            f"t1-nav-accent-{idx:03d}",
            "Nav CTA accent color shift",
            ["color"],
            "low",
            [
                {
                    "target": ".cta",
                    "property": "background",
                    "from": "#38bdf8",
                    "to": color,
                    "delta": "small",
                }
            ],
            [{"label": "color.accent", "severity": "low"}],
            nav_ref,
            nav_template(accent=color),
            viewport=base_vp,
        )

    grid_gaps = ["16px", "24px", "28px", "32px"]
    for idx, gap in enumerate(grid_gaps, 1):
        add_case(
            cases,
            f"t1-grid-gap-{idx:03d}",
            "Grid gap change",
            ["spacing", "layout"],
            "low",
            [
                {
                    "target": ".grid",
                    "property": "gap",
                    "from": "20px",
                    "to": gap,
                    "delta": "small",
                }
            ],
            [{"label": "spacing.gap", "severity": "medium"}],
            grid_ref,
            grid_template(gap=gap),
            viewport=base_vp,
        )

    grid_radii = ["14px", "20px", "24px"]
    for idx, radius in enumerate(grid_radii, 1):
        add_case(
            cases,
            f"t1-grid-radius-{idx:03d}",
            "Grid tile radius change",
            ["shape", "component"],
            "low",
            [
                {
                    "target": ".tile",
                    "property": "border-radius",
                    "from": "18px",
                    "to": radius,
                    "delta": "small",
                }
            ],
            [{"label": "shape.radius", "severity": "medium"}],
            grid_ref,
            grid_template(tile_radius=radius),
            viewport=base_vp,
        )

    # Medium complexity: multiple deltas.
    hero_combo = [
        ("60px", "0 0 8px 0"),
        ("58px", "0 0 12px 0"),
        ("62px", "0 0 20px 0"),
        ("56px", "0 0 10px 0"),
        ("61px", "0 0 6px 0"),
        ("66px", "0 0 18px 0"),
    ]
    for idx, (size, margin) in enumerate(hero_combo, 1):
        add_case(
            cases,
            f"t2-hero-size-margin-{idx:03d}",
            "Hero title size + margin change",
            ["typography", "spacing"],
            "medium",
            [
                {
                    "target": ".hero-title",
                    "property": "font-size",
                    "from": "64px",
                    "to": size,
                    "delta": "small",
                },
                {
                    "target": ".hero-title",
                    "property": "margin-bottom",
                    "from": "16px",
                    "to": margin.split()[2],
                    "delta": "small",
                },
            ],
            [{"label": "typography.size", "severity": "medium"}],
            hero_ref,
            hero_template(title_size=size, title_margin=margin),
            viewport=base_vp,
        )

    palette_pairs = [
        ("#f1f5f9", "#f97316"),
        ("#fff7ed", "#ea580c"),
        ("#fdf2f8", "#db2777"),
        ("#f5f3ff", "#8b5cf6"),
    ]
    for idx, (bg, accent) in enumerate(palette_pairs, 1):
        add_case(
            cases,
            f"t2-hero-palette-{idx:03d}",
            "Hero background + accent shift",
            ["color"],
            "medium",
            [
                {
                    "target": "body",
                    "property": "background",
                    "from": "#f6f4ef",
                    "to": bg,
                    "delta": "small",
                },
                {
                    "target": ".hero-pill",
                    "property": "background",
                    "from": "#c66a2b",
                    "to": accent,
                    "delta": "small",
                },
            ],
            [
                {"label": "color.background", "severity": "low"},
                {"label": "color.accent", "severity": "low"},
            ],
            hero_ref,
            hero_template(bg=bg, accent=accent),
            viewport=base_vp,
        )

    card_shadow_border = [
        ("0 18px 40px rgba(15,23,42,0.18)", "#c7cdd8"),
        ("0 24px 48px rgba(15,23,42,0.2)", "#bfc6d3"),
        ("0 14px 28px rgba(15,23,42,0.16)", "#aeb7c6"),
        ("0 22px 44px rgba(15,23,42,0.22)", "#d1d7e2"),
    ]
    for idx, (shadow, border_color) in enumerate(card_shadow_border, 1):
        add_case(
            cases,
            f"t2-card-shadow-border-{idx:03d}",
            "Card shadow + border change",
            ["color", "component"],
            "medium",
            [
                {
                    "target": ".card",
                    "property": "box-shadow",
                    "from": "0 18px 40px rgba(15,23,42,0.12)",
                    "to": shadow,
                    "delta": "small",
                },
                {
                    "target": ".card",
                    "property": "border-color",
                    "from": "#d6dbe5",
                    "to": border_color,
                    "delta": "tiny",
                },
            ],
            [{"label": "shadow.intensity", "severity": "low"}],
            card_ref,
            card_template(
                shadow=shadow, border=f"1px solid {border_color}"
            ),
            viewport=base_vp,
        )

    stats_align_aside = [
        ("flex-start", "#f1f5f9", "1px solid #cbd5f5"),
        ("center", "#ecfeff", "1px solid #a5f3fc"),
        ("flex-end", "#fff7ed", "1px solid #fed7aa"),
        ("stretch", "#f5f3ff", "1px solid #ddd6fe"),
    ]
    for idx, (align, aside_bg, aside_border) in enumerate(stats_align_aside, 1):
        add_case(
            cases,
            f"t2-stats-align-aside-{idx:03d}",
            "Stats alignment + aside panel change",
            ["alignment", "layout"],
            "medium",
            [
                {
                    "target": ".layout",
                    "property": "align-items",
                    "from": "center",
                    "to": align,
                    "delta": "medium",
                },
                {
                    "target": ".aside",
                    "property": "background",
                    "from": "#ffffff",
                    "to": aside_bg,
                    "delta": "small",
                },
            ],
            [{"label": "alignment.vertical", "severity": "medium"}],
            stats_ref,
            stats_template(align=align, aside_bg=aside_bg, aside_border=aside_border),
            viewport=base_vp,
        )

    grid_columns = ["repeat(2, 1fr)", "repeat(4, 1fr)", "repeat(3, 1fr)", "repeat(2, 1fr)"]
    grid_gaps_combo = ["28px", "24px", "16px", "32px"]
    for idx, (cols, gap) in enumerate(zip(grid_columns, grid_gaps_combo), 1):
        add_case(
            cases,
            f"t2-grid-cols-gap-{idx:03d}",
            "Grid columns + gap change",
            ["layout", "spacing"],
            "medium",
            [
                {
                    "target": ".grid",
                    "property": "grid-template-columns",
                    "from": "repeat(3, 1fr)",
                    "to": cols,
                    "delta": "medium",
                },
                {
                    "target": ".grid",
                    "property": "gap",
                    "from": "20px",
                    "to": gap,
                    "delta": "small",
                },
            ],
            [{"label": "layout.structure", "severity": "medium"}],
            grid_ref,
            grid_template(columns=cols, gap=gap),
            viewport=base_vp,
        )

    nav_icon_accent = [
        ("M12 4v0M12 12v0M12 20v0", "#7dd3fc"),
        ("M4 12h16", "#fbbf24"),
        ("M6 6l12 12M6 18L18 6", "#a7f3d0"),
        ("M3 12h18", "#f472b6"),
    ]
    for idx, (path, color) in enumerate(nav_icon_accent, 1):
        add_case(
            cases,
            f"t2-nav-icon-accent-{idx:03d}",
            "Nav icon + accent change",
            ["icon", "color"],
            "medium",
            [
                {
                    "target": ".mobile-toggle path",
                    "property": "d",
                    "from": "hamburger",
                    "to": "variant",
                    "delta": "small",
                },
                {
                    "target": ".cta",
                    "property": "background",
                    "from": "#38bdf8",
                    "to": color,
                    "delta": "small",
                },
            ],
            [{"label": "icon.mismatch", "severity": "medium"}],
            nav_ref,
            nav_template(icon_path=path, accent=color),
            viewport=base_vp,
        )

    copy_variants = [
        "Compare layout mismatches and fix issues before release.",
        "Detect UI drift and triage fixes instantly.",
        "Find spacing regressions before they ship.",
        "Map diffs to tokens and components fast.",
    ]
    for idx, text in enumerate(copy_variants, 1):
        add_case(
            cases,
            f"t2-hero-copy-{idx:03d}",
            "Hero subtitle copy change",
            ["copy"],
            "medium",
            [
                {
                    "target": ".hero-subtitle",
                    "property": "text",
                    "from": "Compare pixel differences and find issues in seconds, not hours.",
                    "to": text,
                    "delta": "medium",
                }
            ],
            [{"label": "copy.mismatch", "severity": "medium"}],
            hero_ref,
            hero_template().replace(
                "Compare pixel differences and find issues in seconds, not hours.",
                text,
            ),
            viewport=base_vp,
        )

    card_padding_radius = [
        ("28px", "18px"),
        ("36px", "24px"),
        ("40px", "20px"),
        ("30px", "16px"),
    ]
    for idx, (padding, radius) in enumerate(card_padding_radius, 1):
        add_case(
            cases,
            f"t2-card-padding-radius-{idx:03d}",
            "Card padding + radius change",
            ["spacing", "shape"],
            "medium",
            [
                {
                    "target": ".card",
                    "property": "padding",
                    "from": "32px",
                    "to": padding,
                    "delta": "small",
                },
                {
                    "target": ".card",
                    "property": "border-radius",
                    "from": "22px",
                    "to": radius,
                    "delta": "small",
                },
            ],
            [{"label": "spacing.padding", "severity": "medium"}],
            card_ref,
            card_template(padding=padding, radius=radius),
            viewport=base_vp,
        )

    stats_bg_title = [
        ("#f1f5f9", "28px"),
        ("#fff7ed", "32px"),
        ("#f5f3ff", "26px"),
        ("#ecfeff", "31px"),
    ]
    for idx, (bg, title_size) in enumerate(stats_bg_title, 1):
        add_case(
            cases,
            f"t2-stats-bg-title-{idx:03d}",
            "Stats background + title size",
            ["color", "typography"],
            "medium",
            [
                {
                    "target": "body",
                    "property": "background",
                    "from": "#f8fafc",
                    "to": bg,
                    "delta": "small",
                },
                {
                    "target": ".stat strong",
                    "property": "font-size",
                    "from": "30px",
                    "to": title_size,
                    "delta": "small",
                },
            ],
            [{"label": "color.background", "severity": "low"}],
            stats_ref,
            stats_template(bg=bg, title_size=title_size),
            viewport=base_vp,
        )

    # High complexity: structural/layout changes.
    reorder_patterns = [
        [2, 0, 1, 3, 4, 5],
        [1, 0, 2, 4, 3, 5],
        [5, 4, 3, 2, 1, 0],
        [3, 2, 1, 0, 4, 5],
    ]
    area_names = ["a", "b", "c", "d", "e", "f"]
    ref_grid_areas = '"a b c" "d e f"'
    for idx, order in enumerate(reorder_patterns, 1):
        ordered_names = [area_names[i] for i in order]
        impl_grid_areas = (
            f'"{ordered_names[0]} {ordered_names[1]} {ordered_names[2]}" '
            f'"{ordered_names[3]} {ordered_names[4]} {ordered_names[5]}"'
        )
        add_case(
            cases,
            f"t3-grid-reorder-{idx:03d}",
            "Grid tile reorder",
            ["layout", "alignment"],
            "high",
            [
                {
                    "target": ".tile",
                    "property": "order",
                    "from": "0-5",
                    "to": "-".join(str(x) for x in order),
                    "delta": "large",
                }
            ],
            [{"label": "layout.order", "severity": "high"}],
            grid_template(grid_areas=ref_grid_areas, tile_areas=area_names),
            grid_template(grid_areas=impl_grid_areas, tile_areas=area_names),
            viewport=base_vp,
        )

    column_sets = ["repeat(2, 1fr)", "repeat(4, 1fr)", "repeat(1, 1fr)", "repeat(5, 1fr)"]
    for idx, cols in enumerate(column_sets, 1):
        add_case(
            cases,
            f"t3-grid-columns-{idx:03d}",
            "Grid column count change",
            ["layout", "spacing"],
            "high",
            [
                {
                    "target": ".grid",
                    "property": "grid-template-columns",
                    "from": "repeat(3, 1fr)",
                    "to": cols,
                    "delta": "large",
                }
            ],
            [{"label": "layout.structure", "severity": "high"}],
            grid_ref,
            grid_template(columns=cols, gap="28px"),
            viewport=base_vp,
        )

    missing_tiles = [5, 4, 3]
    for idx, count in enumerate(missing_tiles, 1):
        add_case(
            cases,
            f"t3-grid-missing-{idx:03d}",
            "Missing grid tiles",
            ["layout", "component"],
            "high",
            [
                {
                    "target": ".tile",
                    "property": "count",
                    "from": "6",
                    "to": str(count),
                    "delta": "large",
                }
            ],
            [{"label": "layout.missing_element", "severity": "high"}],
            grid_ref,
            grid_template(tile_count=count),
            viewport=base_vp,
        )

    multi_mutations = [
        {
            "target": ".mobile-toggle path",
            "property": "d",
            "from": "hamburger",
            "to": "variant",
            "delta": "small",
        }
    ]
    add_case(
        cases,
        "t3-multi-viewport-001",
        "Mobile nav icon mismatch",
        ["icon", "layout"],
        "high",
        multi_mutations,
        [{"label": "icon.mismatch", "severity": "medium"}],
        nav_template(
            icon_path="M3 6h18M3 12h18M3 18h18",
            mobile_icon_path="M3 6h18M3 12h18M3 18h18",
        ),
        nav_template(
            icon_path="M3 6h18M3 12h18M3 18h18",
            mobile_icon_path="M12 4v0M12 12v0M12 20v0",
        ),
        viewports=viewports,
        assertions_by_viewport={
            "desktop": {"pixel_regions_min": 0, "similarity_max": 1.0},
            "mobile": default_assertions(["icon", "layout"], "high", multi_mutations),
        },
    )

    mobile_icons = [
        "M12 4v0M12 12v0M12 20v0",
        "M4 12h16",
        "M6 6l12 12M6 18L18 6",
    ]
    for idx, icon in enumerate(mobile_icons, 1):
        nav_mutations = [
            {
                "target": ".mobile-toggle path",
                "property": "d",
                "from": "hamburger",
                "to": "variant",
                "delta": "small",
            }
        ]
        add_case(
            cases,
            f"t3-nav-multi-viewport-{idx:03d}",
            "Mobile nav icon mismatch",
            ["icon", "layout"],
            "high",
            nav_mutations,
            [{"label": "icon.mismatch", "severity": "medium"}],
            nav_template(
                icon_path="M3 6h18M3 12h18M3 18h18",
                mobile_icon_path="M3 6h18M3 12h18M3 18h18",
            ),
            nav_template(
                icon_path="M3 6h18M3 12h18M3 18h18",
                mobile_icon_path=icon,
            ),
            viewports=viewports,
            assertions_by_viewport={
                "desktop": {"pixel_regions_min": 0, "similarity_max": 1.0},
                "mobile": default_assertions(["icon", "layout"], "high", nav_mutations),
            },
        )

    hero_layouts = [
        ("1fr 1fr", "32px", "0 16px 28px rgba(0,0,0,0.12)"),
        ("1.4fr 0.6fr", "56px", "0 10px 24px rgba(0,0,0,0.08)"),
        ("1fr", "24px", "0 20px 32px rgba(0,0,0,0.16)"),
    ]
    for idx, (cols, gap, shadow) in enumerate(hero_layouts, 1):
        add_case(
            cases,
            f"t3-hero-layout-{idx:03d}",
            "Hero layout structure change",
            ["layout", "spacing"],
            "high",
            [
                {
                    "target": ".hero",
                    "property": "grid-template-columns",
                    "from": "1.2fr 0.8fr",
                    "to": cols,
                    "delta": "large",
                },
                {
                    "target": ".hero",
                    "property": "gap",
                    "from": "48px",
                    "to": gap,
                    "delta": "medium",
                },
                {
                    "target": ".panel",
                    "property": "box-shadow",
                    "from": "0 20px 40px rgba(0,0,0,0.08)",
                    "to": shadow,
                    "delta": "small",
                },
            ],
            [{"label": "layout.structure", "severity": "high"}],
            hero_ref,
            hero_template(columns=cols, gap=gap, panel_shadow=shadow),
            viewport=base_vp,
        )

    hero_panel_hidden = [
        ("none", "Panel removed"),
        ("none", "Panel hidden variant"),
        ("none", "Panel collapsed"),
    ]
    for idx, (display, title) in enumerate(hero_panel_hidden, 1):
        add_case(
            cases,
            f"t3-hero-panel-hidden-{idx:03d}",
            title,
            ["layout", "component"],
            "high",
            [
                {
                    "target": ".panel",
                    "property": "display",
                    "from": "block",
                    "to": "none",
                    "delta": "large",
                }
            ],
            [{"label": "layout.missing_element", "severity": "high"}],
            hero_ref,
            hero_template(panel_display=display),
            viewport=base_vp,
        )

    for case in cases:
        case_id = case["case_id"]
        case_dir = out_dir / case_id
        case_dir.mkdir(parents=True, exist_ok=True)
        (case_dir / "ref.html").write_text(case.pop("ref_html"))
        (case_dir / "impl.html").write_text(case.pop("impl_html"))
        meta = case_meta(**case)
        (case_dir / "meta.json").write_text(json.dumps(meta, indent=2) + "\n")


if __name__ == "__main__":
    main()
