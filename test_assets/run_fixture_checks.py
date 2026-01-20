#!/usr/bin/env python3
import argparse
import json
import shlex
import subprocess
from pathlib import Path
from datetime import datetime, timezone


def load_cases(fixtures_dir: Path):
    for case_dir in sorted(fixtures_dir.iterdir()):
        if not case_dir.is_dir():
            continue
        meta_path = case_dir / "meta.json"
        if not meta_path.exists():
            continue
        yield case_dir, meta_path


def parse_cmd(cmd_str):
    return shlex.split(cmd_str)


def extract_json(text):
    if not text:
        return None
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        pass
    indices = [idx for idx, ch in enumerate(text) if ch == "{"]
    for idx in reversed(indices):
        chunk = text[idx:].strip()
        try:
            return json.loads(chunk)
        except json.JSONDecodeError:
            continue
    return None


def run_compare(
    cmd,
    ref_path,
    impl_path,
    threshold=None,
    ref_type=None,
    impl_type=None,
    viewport=None,
):
    compare_cmd = list(cmd)
    compare_cmd += [
        "compare",
        "--ref",
        str(ref_path),
        "--impl",
        str(impl_path),
        "--format",
        "json",
    ]
    if ref_type:
        compare_cmd += ["--ref-type", ref_type]
    if impl_type:
        compare_cmd += ["--impl-type", impl_type]
    if viewport:
        compare_cmd += ["--viewport", viewport]
    if threshold is not None:
        compare_cmd += ["--threshold", str(threshold)]
    result = subprocess.run(
        compare_cmd,
        capture_output=True,
        text=True,
    )
    stdout = result.stdout.strip()
    payload = extract_json(stdout)
    if payload is None:
        payload = extract_json(result.stderr.strip())
    return result.returncode, payload, result.stderr.strip()


def get_metric(payload, key):
    metrics = payload.get("metrics") if payload else None
    if not metrics:
        return None
    return metrics.get(key)


def check_assertions(case_id, payload, assertions, viewport_name=None):
    failures = []
    if not payload:
        return [format_case_id(case_id, viewport_name) + ": missing compare payload"]
    if payload.get("mode") == "error":
        msg = payload.get("error", {}).get("message", "unknown error")
        return [format_case_id(case_id, viewport_name) + f": error payload: {msg}"]
    if payload.get("mode") != "compare":
        return [
            format_case_id(case_id, viewport_name)
            + f": unexpected mode {payload.get('mode')}"
        ]

    similarity = payload.get("similarity")
    pixel = get_metric(payload, "pixel")
    color = get_metric(payload, "color")
    typography = get_metric(payload, "typography")
    layout = get_metric(payload, "layout")
    content = get_metric(payload, "content")

    if "pixel_regions_min" in assertions:
        diff_regions = pixel.get("diffRegions") if pixel else None
        if diff_regions is not None:
            count = len(diff_regions)
            if count < assertions["pixel_regions_min"]:
                failures.append(
                    format_case_id(case_id, viewport_name)
                    + f": pixel diff regions {count} < {assertions['pixel_regions_min']}"
                )

    if "color_diffs_min" in assertions:
        diffs = color.get("diffs") if color else None
        if diffs is not None:
            count = len(diffs)
            if count < assertions["color_diffs_min"]:
                failures.append(
                    format_case_id(case_id, viewport_name)
                    + f": color diffs {count} < {assertions['color_diffs_min']}"
                )

    if "similarity_max" in assertions and similarity is not None:
        if similarity > assertions["similarity_max"]:
            failures.append(
                format_case_id(case_id, viewport_name)
                + f": similarity {similarity:.4f} > {assertions['similarity_max']}"
            )

    if "color_score_max" in assertions:
        score = color.get("score") if color else None
        if score is not None and score > assertions["color_score_max"]:
            failures.append(
                format_case_id(case_id, viewport_name)
                + f": color score {score:.4f} > {assertions['color_score_max']}"
            )

    if "typography_score_max" in assertions:
        score = typography.get("score") if typography else None
        if score is not None and score > assertions["typography_score_max"]:
            failures.append(
                format_case_id(case_id, viewport_name)
                + f": typography score {score:.4f} > {assertions['typography_score_max']}"
            )

    if "layout_score_max" in assertions:
        score = layout.get("score") if layout else None
        if score is not None and score > assertions["layout_score_max"]:
            failures.append(
                format_case_id(case_id, viewport_name)
                + f": layout score {score:.4f} > {assertions['layout_score_max']}"
            )

    if "content_score_max" in assertions:
        score = content.get("score") if content else None
        if score is not None and score > assertions["content_score_max"]:
            failures.append(
                format_case_id(case_id, viewport_name)
                + f": content score {score:.4f} > {assertions['content_score_max']}"
            )

    return failures


def format_case_id(case_id, viewport_name):
    if viewport_name:
        return f"{case_id}::{viewport_name}"
    return case_id


def main():
    parser = argparse.ArgumentParser(description="Run fixture checks with DPC.")
    parser.add_argument(
        "--fixtures-dir",
        default="test_assets/fixtures",
        help="Directory with rendered fixture PNGs.",
    )
    parser.add_argument(
        "--fixtures-src-dir",
        default="test_assets/fixtures_src",
        help="Directory with fixture HTML sources.",
    )
    parser.add_argument(
        "--cmd",
        default="dpc",
        help='Command to run DPC (e.g. "cargo run --quiet --").',
    )
    parser.add_argument(
        "--threshold",
        type=float,
        default=None,
        help="Optional threshold override for dpc compare.",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=None,
        help="Limit number of cases to run.",
    )
    parser.add_argument(
        "--report-out",
        default=None,
        help="Write per-case results as JSONL to this path.",
    )
    parser.add_argument(
        "--use-html",
        action="store_true",
        help="Use ref.html/impl.html via Playwright (semantic metrics).",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Exit nonzero on any assertion failure.",
    )
    args = parser.parse_args()

    fixtures_dir = Path(args.fixtures_dir)
    fixtures_src_dir = Path(args.fixtures_src_dir)
    cmd = parse_cmd(args.cmd)

    cases = list(load_cases(fixtures_dir))
    if args.limit:
        cases = cases[: args.limit]
    if not cases:
        raise SystemExit(f"No cases found in {fixtures_dir}")

    failures = []
    errors = 0
    total = len(cases)
    if args.report_out:
        report_path = Path(args.report_out)
    else:
        ts = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
        report_path = Path("test_assets/reports") / f"fixtures_{ts}.jsonl"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_file = report_path.open("w")
    print(f"Report: {report_path}")

    for idx, (case_dir, meta_path) in enumerate(cases, start=1):
        meta = json.loads(meta_path.read_text())
        case_id = meta.get("case_id", case_dir.name)
        assertions = meta.get("assertions", {})
        assertions_by_viewport = meta.get("assertions_by_viewport", {})

        is_multi = "viewports" in meta
        if args.use_html:
            case_src_dir = fixtures_src_dir / case_dir.name
            ref_path = case_src_dir / "ref.html"
            impl_path = case_src_dir / "impl.html"
            if not ref_path.exists() or not impl_path.exists():
                errors += 1
                failures.append(
                    f"{case_id}: missing HTML source in {case_src_dir}"
                )
                if report_file:
                    report_file.write(
                        json.dumps(
                            {
                                "case_id": case_id,
                                "status": "error",
                                "error": "missing HTML source",
                                "timestamp": datetime.now(timezone.utc).isoformat(),
                            }
                        )
                        + "\n"
                    )
                continue
            ref_type = "url"
            impl_type = "url"
            if is_multi:
                viewports = meta["viewports"]
            else:
                viewports = [meta.get("viewport")] if meta.get("viewport") else []
        else:
            ref_type = None
            impl_type = None
            viewports = [None]

        if args.use_html:
            ref_path = ref_path.resolve().as_uri()
            impl_path = impl_path.resolve().as_uri()

        print(f"[{idx}/{total}] {case_id}")
        for vp in viewports:
            viewport_name = vp.get("name") if isinstance(vp, dict) else None
            if args.use_html:
                viewport_arg = (
                    f"{vp['width']}x{vp['height']}" if vp else None
                )
                ref_input = ref_path
                impl_input = impl_path
            else:
                viewport_arg = None
                if is_multi:
                    suffix = f".{viewport_name}" if viewport_name else ".desktop"
                    ref_input = case_dir / f"ref{suffix}.png"
                    impl_input = case_dir / f"impl{suffix}.png"
                else:
                    ref_input = case_dir / "ref.png"
                    impl_input = case_dir / "impl.png"

            code, payload, stderr = run_compare(
                cmd,
                ref_input,
                impl_input,
                threshold=args.threshold,
                ref_type=ref_type,
                impl_type=impl_type,
                viewport=viewport_arg,
            )
            if payload is None:
                errors += 1
                failures.append(
                    f"{format_case_id(case_id, viewport_name)}: no JSON output (code {code}) {stderr}"
                )
                if report_file:
                    report_file.write(
                        json.dumps(
                            {
                                "case_id": case_id,
                                "viewport": viewport_name,
                                "status": "error",
                                "error": stderr or "no json output",
                                "timestamp": datetime.now(timezone.utc).isoformat(),
                            }
                        )
                        + "\n"
                    )
                continue

            assertions_override = assertions_by_viewport.get(viewport_name)
            if assertions_override is not None:
                assertions_to_use = assertions_override
            else:
                assertions_to_use = assertions

            case_failures = check_assertions(
                case_id, payload, assertions_to_use, viewport_name
            )
            failures.extend(case_failures)
            if report_file:
                report_file.write(
                    json.dumps(
                        {
                            "case_id": case_id,
                            "viewport": viewport_name,
                            "status": "ok" if not case_failures else "fail",
                            "failures": case_failures,
                            "assertions": assertions_to_use,
                            "meta": {
                                "category": meta.get("category", []),
                                "complexity": meta.get("complexity"),
                                "mutations": meta.get("mutations", []),
                                "expectations": meta.get("expectations", []),
                            },
                            "result": payload,
                            "timestamp": datetime.now(timezone.utc).isoformat(),
                        }
                    )
                    + "\n"
                )

    print(f"\nTotal: {total}")
    print(f"Failures: {len(failures)}")
    print(f"Errors: {errors}")
    if failures:
        print("\nFailures:")
        for failure in failures[:50]:
            print(f"- {failure}")
        if len(failures) > 50:
            print(f"... and {len(failures) - 50} more")

    if report_file:
        report_file.close()

    if args.strict and (failures or errors):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
