#!/usr/bin/env python3
import argparse
import json
from pathlib import Path

from playwright.sync_api import sync_playwright


def load_cases(src_dir: Path):
    for case_dir in sorted(src_dir.iterdir()):
        if not case_dir.is_dir():
            continue
        meta_path = case_dir / "meta.json"
        ref_path = case_dir / "ref.html"
        impl_path = case_dir / "impl.html"
        if meta_path.exists() and ref_path.exists() and impl_path.exists():
            yield case_dir, meta_path, ref_path, impl_path


def resolve_viewports(meta):
    if "viewports" in meta:
        return True, meta["viewports"]
    if "viewport" in meta:
        vp = meta["viewport"]
        vp = dict(vp)
        vp.setdefault("name", "default")
        return False, [vp]
    raise ValueError("meta.json missing viewport or viewports")


def take_screenshots(page, html_path: Path, out_path: Path):
    page.goto(html_path.resolve().as_uri(), wait_until="networkidle")
    page.add_style_tag(
        content="*{animation:none!important;transition:none!important;}"
    )
    page.wait_for_timeout(50)
    page.screenshot(path=str(out_path), full_page=True)


def main():
    parser = argparse.ArgumentParser(description="Render fixture PNGs from HTML pairs.")
    parser.add_argument(
        "--src-dir",
        default="test_assets/fixtures_src",
        help="Directory containing case folders.",
    )
    parser.add_argument(
        "--out-dir",
        default="test_assets/fixtures",
        help="Directory to write PNGs and meta.json.",
    )
    args = parser.parse_args()

    src_dir = Path(args.src_dir)
    out_dir = Path(args.out_dir)

    cases = list(load_cases(src_dir))
    if not cases:
        raise SystemExit(f"No cases found in {src_dir}")

    with sync_playwright() as p:
        browser = p.chromium.launch()
        try:
            total = len(cases)
            for idx, (case_dir, meta_path, ref_html, impl_html) in enumerate(
                cases, start=1
            ):
                meta = json.loads(meta_path.read_text())
                case_id = meta.get("case_id", case_dir.name)
                is_multi, viewports = resolve_viewports(meta)

                case_out = out_dir / case_id
                case_out.mkdir(parents=True, exist_ok=True)
                (case_out / "meta.json").write_text(
                    json.dumps(meta, indent=2) + "\n"
                )
                print(f"[{idx}/{total}] Rendering {case_id}...")

                for viewport in viewports:
                    name = viewport.get("name", "default")
                    suffix = f".{name}" if is_multi else ""
                    page = browser.new_page(viewport=viewport)
                    try:
                        take_screenshots(
                            page, ref_html, case_out / f"ref{suffix}.png"
                        )
                        take_screenshots(
                            page, impl_html, case_out / f"impl{suffix}.png"
                        )
                    finally:
                        page.close()
        finally:
            browser.close()


if __name__ == "__main__":
    main()
