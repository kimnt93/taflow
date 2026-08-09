#!/usr/bin/env python3
"""Render one highest-priority correctness oracle for every TAFlow class."""
from __future__ import annotations

import importlib.metadata
import json
from collections import defaultdict
from pathlib import Path

from registry import build_registry


HERE = Path(__file__).parent
PRIORITY = {"TA-Lib": 1, "NumPy": 2, "Polars": 3, "pandas": 4,
            "pandas-ta-classic": 5, "Wickra": 6, "smartmoneyconcepts": 7,
            "self": 99}
URLS = {
    "TA-Lib": "https://ta-lib.github.io/ta-lib-python/funcs.html",
    "NumPy": "https://numpy.org/doc/stable/reference/ufuncs.html",
    "Polars": "https://docs.pola.rs/api/python/stable/reference/expressions/index.html",
    "pandas": "https://pandas.pydata.org/docs/reference/window.html",
    "pandas-ta-classic": "https://xgboosted.github.io/pandas-ta-classic/indicators.html",
    "smartmoneyconcepts": "https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6",
    "Wickra": "https://docs.wickra.org/Indicators/Indicator-Rmi",
    "self": "",
}
PACKAGES = {"TA-Lib": "TA-Lib", "NumPy": "numpy", "Polars": "polars", "pandas": "pandas",
            "pandas-ta-classic": "pandas-ta-classic",
            "Wickra": "wickra",
            "smartmoneyconcepts": "smartmoneyconcepts"}
API_NAMES = {
    "anchored_vwap": "pandas.core.groupby.SeriesGroupBy.cumsum",
    "awesome_oscillator": "pandas_ta_classic.ao",
    "log_return": "pandas_ta_classic.log_return",
    "force_index": "pandas_ta_classic.efi",
    "hull_moving_average": "pandas_ta_classic.hma",
    "volume_weighted_moving_average": "pandas_ta_classic.vwma",
    "zero_lag_exponential_moving_average": "pandas_ta_classic.zlma",
    "donchian_channels": "pandas_ta_classic.donchian",
    "fisher_transform": "pandas_ta_classic.fisher",
    "chaikin_money_flow": "pandas_ta_classic.cmf",
    "detrended_price_oscillator": "pandas_ta_classic.dpo",
    "mcginley_dynamic": "pandas_ta_classic.mcgd",
    "variable_index_dynamic_average": "pandas_ta_classic.vidya",
    "laguerre_relative_strength_index": "pandas_ta_classic.lrsi",
    "laguerre_rsi": "pandas_ta_classic.lrsi",
    "jurik_moving_average": "pandas_ta_classic.jma",
    "even_better_sinewave": "pandas_ta_classic.ebsw",
    "schaff_trend_cycle": "pandas_ta_classic.stc",
    "klinger_volume_oscillator": "pandas_ta_classic.kvo",
    "tom_de_mark_sequential": "pandas_ta_classic.td_seq",
    "rmi": "wickra.RMI",
}
API_URLS = {
    "anchored_vwap": (
        "https://pandas.pydata.org/docs/reference/api/"
        "pandas.core.groupby.SeriesGroupBy.cumsum.html"
    ),
    "laguerre_rsi": (
        "https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/"
        "pandas_ta_classic/momentum/lrsi.py"
    ),
}


def version(source: str, external_versions: dict[str, str]) -> str:
    package = PACKAGES.get(source)
    if not package:
        return "repository invariant"
    if source == "smartmoneyconcepts":
        return f"{external_versions.get(package, '?')} @ 1b62fd6c"
    try:
        return importlib.metadata.version(package)
    except importlib.metadata.PackageNotFoundError:
        return external_versions.get(package, "unknown")


def main() -> None:
    primary = json.loads((HERE / "report.json").read_text())
    external_doc = json.loads((HERE / "EXTERNAL_ORACLES.json").read_text())
    external = external_doc["rows"]
    registry = build_registry()
    snake_by_key = {key: spec.snake for key, spec in registry.items()}
    class_by_snake = {spec.snake: spec.cls.__name__ for spec in registry.values() if spec.cls}
    canonical_snake_by_class = {
        spec.cls.__name__: spec.snake for spec in registry.values() if spec.cls
    }
    external_by_function: dict[str, list[dict]] = defaultdict(list)
    for row in external:
        # External checks use descriptive names while CHECK.md retains a few
        # compatibility keys (for example ``kvo``). Resolve through the live
        # class so both identities select the same evidence.
        from registry import resolve_class
        cls = resolve_class(row["function"])
        key = canonical_snake_by_class.get(cls.__name__) if cls else None
        if not key:
            key = row["function"]
        external_by_function[key].append(row)

    selected: list[dict] = []
    for row in primary:
        snake = snake_by_key[row["function"]]
        candidates = [(PRIORITY[row["oracle"]], row["oracle"], [row])]
        for source in {item["oracle"] for item in external_by_function.get(snake, [])}:
            candidates.append((PRIORITY[source], source,
                               [item for item in external_by_function[snake]
                                if item["oracle"] == source]))
        _, source, evidence = min(candidates, key=lambda item: item[0])
        if source == "self" or source == row["oracle"]:
            check = row.get("batch_vs_oracle") or {}
            lifecycle = bool(row.get("continue_vs_batch_bitwise")) and all(
                row.get("chunk_invariance", {}).values())
            passed = lifecycle and (source == "self" or bool(check.get("passed")))
            evidence = [{"output": "all", "passed": passed,
                         "expected_difference": False,
                         "max_abs_error": check.get("max_abs_error", 0.0),
                         "nan_mismatches": check.get("nan_mismatches", 0),
                         "note": ("cold/warm/chunk/reset invariant; no external oracle"
                                  if source == "self" else
                                  "external parity plus bitwise lifecycle invariance")}]
        for item in evidence:
            verdict = ("MATCH" if item["passed"] else
                       "VARIANT" if item.get("expected_difference") else "FAIL")
            evidence_api = item.get("note", "") if source == "NumPy" else ""
            oracle_api = (row["function"] if source == "TA-Lib" else
                          evidence_api if evidence_api.startswith("numpy.") else
                          API_NAMES.get(snake, f"{source}.{snake}"))
            selected.append({"class": class_by_snake[snake], "snake": snake,
                             "oracle_api": oracle_api, "output": item["output"],
                             "source": source, "version": version(source, external_doc["versions"]),
                             "url": API_URLS.get(snake, URLS[source]), "verdict": verdict,
                             "error": float(item.get("max_abs_error", 0.0)),
                             "nan": int(item.get("nan_mismatches", 0)),
                             "note": item.get("error") or item.get("note", "")})

    # Some canonical-only classes are not present in the legacy TA-Lib report
    # but do have independent evidence from the external-oracle harness. Keep
    # those rows in the priority-selected report instead of leaving their
    # correctness blank in the checklist.
    seen = {(row["class"], row["output"]) for row in selected}
    extra_classes = {
        "rolling_percentile": "RollingPercentile",
        "rolling_interquartile_range": "RollingInterquartileRange",
    }
    for snake, evidence in external_by_function.items():
        class_name = class_by_snake.get(snake) or extra_classes.get(snake)
        if not class_name:
            continue
        source = min((item["oracle"] for item in evidence), key=lambda item: PRIORITY[item])
        for item in (item for item in evidence if item["oracle"] == source):
            if (class_name, item["output"]) in seen:
                continue
            verdict = ("MATCH" if item["passed"] else
                       "VARIANT" if item.get("expected_difference") else "FAIL")
            selected.append({
                "class": class_name,
                "snake": snake,
                "oracle_api": API_NAMES.get(snake, f"{source}.{snake}"),
                "output": item["output"],
                "source": source,
                "version": version(source, external_doc["versions"]),
                "url": API_URLS.get(snake, URLS[source]),
                "verdict": verdict,
                "error": float(item.get("max_abs_error", 0.0)),
                "nan": int(item.get("nan_mismatches", 0)),
                "note": item.get("error") or item.get("note", ""),
            })
            seen.add((class_name, item["output"]))

    counts = {name: sum(r["verdict"] == name for r in selected)
              for name in ("MATCH", "VARIANT", "FAIL")}
    invariant = sum(r["source"] == "self" for r in selected)
    lines = ["# Priority-selected correctness sources", "",
             "One oracle is selected per indicator using: **TA-Lib > NumPy > Polars > pandas > "
             "pandas-ta-classic > pinned GitHub**. `VARIANT` is a documented semantic "
             "difference, not a failed comparison; `INVARIANT` rows have no external oracle.", "",
             f"Matches: **{counts['MATCH']}** | Documented variants: **{counts['VARIANT']}** | "
             f"Self-invariant outputs: **{invariant}** | Failures: **{counts['FAIL']}**", "",
             "| TAFlow class ↔ oracle API | Output | Selected source | Version | Verdict | Max error | NaN | Note |",
             "|---|---|---|---|---:|---:|---:|---|"]
    for r in sorted(selected, key=lambda x: (x["class"], x["output"])):
        source = f"[{r['source']}]({r['url']})" if r["url"] else "native invariant"
        verdict = "INVARIANT" if r["source"] == "self" and r["verdict"] == "MATCH" else r["verdict"]
        lines.append(f"| `{r['class']}` ↔ `{r['oracle_api']}` | `{r['output']}` | {source} | "
                     f"`{r['version']}` | {verdict} | `{r['error']:.3e}` | {r['nan']} | {r['note']} |")
    (HERE / "SOURCE_COMPARISON.md").write_text("\n".join(lines) + "\n")
    (HERE / "SOURCE_COMPARISON.json").write_text(json.dumps(selected, indent=2) + "\n")
    print(f"wrote SOURCE_COMPARISON: {len(selected)} outputs, {counts['FAIL']} failures")
    if counts["FAIL"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
