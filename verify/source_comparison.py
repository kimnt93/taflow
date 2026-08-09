#!/usr/bin/env python3
"""Merge generated oracle artifacts without overstating self-consistency.

``verify.py`` owns TA-Lib/pandas parity and lifecycle invariance;
``external_oracles.py`` owns pandas-ta-classic, Polars, and SMC comparisons.
This script only renders their recorded results. It performs no hidden checks,
does not catch oracle failures, and never labels a native self-check as external
correctness evidence.
"""
from __future__ import annotations

import json
from pathlib import Path


HERE = Path(__file__).parent

EXTERNAL_TO_PRIMARY = {
    "abs": "math_abs",
    "ewm_stddev": "ewm_std",
    "ewm_variance": "ewm_var",
    "jurik_moving_average": "jma",
    "klinger_volume_oscillator": "kvo",
    "mcginley_dynamic": "mcginley",
    "tom_de_mark_sequential": "td_sequential",
    "variable_index_dynamic_average": "vidya",
}


def _error(check: dict | None) -> tuple[float, int]:
    if not check:
        return float("inf"), 0
    return float(check.get("max_abs_error", 0.0)), int(check.get("nan_mismatches", 0))


def main() -> None:
    primary = json.loads((HERE / "report.json").read_text())
    external = json.loads((HERE / "EXTERNAL_ORACLES.json").read_text())["rows"]
    externally_compared = {
        EXTERNAL_TO_PRIMARY.get(result["function"], result["function"])
        for result in external
    }
    rows: list[dict] = []

    for result in primary:
        oracle = result["oracle"]
        batch = result.get("batch_vs_oracle")
        lifecycle = bool(result.get("continue_vs_batch_bitwise")) and all(
            result.get("chunk_invariance", {}).values())
        if oracle == "self":
            if result["function"] in externally_compared:
                # The per-output external rows below are the stronger evidence;
                # avoid also counting this function as "self-invariant only".
                continue
            verdict = "INVARIANT" if lifecycle else "FAIL"
            note = "native batch/append/chunk consistency only; no external oracle"
            error, nan_mismatches = 0.0, 0
        else:
            passed = bool(batch and batch.get("passed")) and lifecycle
            verdict = "MATCH" if passed else "FAIL"
            note = "external parity plus bitwise lifecycle/chunk invariance"
            error, nan_mismatches = _error(batch)
        rows.append({
            "function": result["function"],
            "python": result["taflow_class"],
            "output": "all",
            "source": oracle,
            "verdict": verdict,
            "max_abs_error": error,
            "nan_mismatches": nan_mismatches,
            "note": note,
        })

    for result in external:
        verdict = ("MATCH" if result["passed"] else
                   "VARIANT" if result["expected_difference"] else "FAIL")
        rows.append({
            "function": result["function"],
            "python": result["function"],
            "output": result["output"],
            "source": result["oracle"],
            "verdict": verdict,
            "max_abs_error": result["max_abs_error"],
            "nan_mismatches": result["nan_mismatches"],
            "note": result.get("error") or result.get("note", ""),
        })

    failures = sum(row["verdict"] == "FAIL" for row in rows)
    matches = sum(row["verdict"] == "MATCH" for row in rows)
    variants = sum(row["verdict"] == "VARIANT" for row in rows)
    invariants = sum(row["verdict"] == "INVARIANT" for row in rows)
    lines = [
        "# Source-labelled correctness comparison", "",
        "Generated from `report.json` and `EXTERNAL_ORACLES.json` by "
        "`source_comparison.py`. `INVARIANT` means native lifecycle "
        "self-consistency, not external numerical validation.", "",
        f"Matches: **{matches}** | Documented variants: **{variants}** | "
        f"Self-invariant only: **{invariants}** | Failures: **{failures}**", "",
        "| Python | Function | Output | Source | Verdict | Max error | NaN mismatches | Note |",
        "|---|---|---|---|---:|---:|---:|---|",
    ]
    for row in sorted(rows, key=lambda item: (
            item["verdict"], item["source"], item["function"], item["output"])):
        lines.append(
            f"| `{row['python']}` | `{row['function']}` | `{row['output']}` | "
            f"{row['source']} | {row['verdict']} | `{row['max_abs_error']:.3e}` | "
            f"{row['nan_mismatches']} | {row['note']} |"
        )
    (HERE / "SOURCE_COMPARISON.md").write_text("\n".join(lines) + "\n")
    print(f"wrote SOURCE_COMPARISON.md: {len(rows)} rows, {failures} failures")
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
