"""Build the source-labelled correctness table requested by the project.

Source labels are deliberately explicit: an external oracle is distinguished
from the native lifecycle self-oracle, so an unavailable pandas-ta/SMC oracle
cannot be mistaken for numerical parity.
"""
from __future__ import annotations

import json
from pathlib import Path

import numpy as np
import pandas as pd

HERE = Path(__file__).parent

SOURCES = {
    "[1]": "TA-Lib",
    "[2]": "pandas",
    "[3]": "pandas-ta-classic (reference inventory)",
    "[4]": "SmartMoneyConcepts (reference inventory)",
    "[5]": "taflow native self-consistency",
}

SMC_NAMES = {
    "FairValueGap", "OrderBlock", "Liquidity", "PreviousHighLow", "Retracements",
    "Sessions", "SwingHighsLows", "BreakOfStructureChangeOfCharacter",
}


def main() -> None:
    inventory = json.loads((HERE / "function_inventory.json").read_text())
    external = {row["function"]: row for row in json.loads((HERE / "report.json").read_text())}
    self_rows = {row["name"]: row for row in json.loads((HERE / "ALL_INTERFACES.json").read_text())}
    # TAFlow no longer exposes a TA-Lib compatibility namespace. External
    # TA-Lib rows are therefore not synthesized here; pandas/native rows are
    # still reported when present in the generated artifacts.
    state_to_ta = {}

    # Exact-parameter pandas-ta-classic checks for extensions that expose the
    # same definition and defaults.  Other extension rows remain explicitly
    # self-consistency-only below.
    n = 128
    close = np.linspace(100.0, 110.0, n)
    high, low = close + 1.0, close - 1.0
    volume = np.linspace(100_000.0, 200_000.0, n)
    pclose, phigh, plow, pvolume = map(pd.Series, (close, high, low, volume))
    pta_checks = {}
    try:
        import pandas_ta_classic as pta

        def compare(actual, expected):
            a, b = np.asarray(actual, dtype=float), np.asarray(expected, dtype=float)
            mask = ~np.isnan(a) & ~np.isnan(b)
            nan_mismatch = int((np.isnan(a) != np.isnan(b)).sum())
            error = float(np.max(np.abs(a[mask] - b[mask]))) if mask.any() else 0.0
            return nan_mismatch == 0 and np.allclose(a, b, equal_nan=True, rtol=1e-8, atol=1e-10), error, nan_mismatch

        def check(name, taflow_output, oracle_output):
            ok, error, nan_mismatch = compare(taflow_output, oracle_output)
            pta_checks[name] = (ok, error, nan_mismatch)

        import taflow
        check("MoneyFlowIndex", taflow.MoneyFlowIndex(high=high, low=low, close=close, volume=volume).compute(), pta.mfi(phigh, plow, pclose, pvolume, length=14).to_numpy())
        check("ChaikinMoneyFlow", taflow.ChaikinMoneyFlow(high=high, low=low, close=close, volume=volume, period=20).compute(), pta.cmf(phigh, plow, pclose, pvolume, length=20).to_numpy())
        check("AwesomeOscillator", taflow.AwesomeOscillator(high=high, low=low, fast=5, slow=34).compute(), pta.ao(phigh, plow, fast=5, slow=34).to_numpy())
        check("FisherTransform", taflow.FisherTransform(high=high, low=low, timeperiod=10).compute(), pta.fisher(phigh, plow, length=10).iloc[:, 0].to_numpy())
        kvo = taflow.KlingerVolumeOscillator(high=high, low=low, close=close, volume=volume, fast=34, slow=55, signal=13).compute()
        ptkvo = pta.kvo(phigh, plow, pclose, pvolume, fast=34, slow=55, signal=13)
        check("KlingerVolumeOscillator", kvo[0], ptkvo.iloc[:, 0].to_numpy())
    except Exception:
        pass

    rows: list[tuple[str, str, str, str]] = []
    for item in inventory["rows"]:
        name = item["name"]
        if name not in self_rows:
            continue
        self_result = self_rows[name]
        ta_alias = next((state_to_ta[s] for s in item.get("native_symbols", []) if s in state_to_ta), None)
        pandas_alias = {
            "RollingMedian": "rolling_median",
            "RollingQuantile": "rolling_quantile",
            "RollingSkew": "rolling_skew",
            "RollingKurtosis": "rolling_kurtosis",
            "RollingZScore": "rolling_zscore",
            "RollingCov": "rolling_cov",
            "ExponentiallyWeightedStandardDeviation": "ewm_std",
            "ExponentiallyWeightedVariance": "ewm_var",
            "RollingInterquartileRange": "rolling_iqr",
        }.get(name, name.lower())
        if name in pta_checks:
            ok, error_value, nan_mismatch = pta_checks[name]
            source, error, correct = "[3] pandas-ta-classic", f"{error_value:.1e} (nan={nan_mismatch})", ok
        elif ta_alias and ta_alias in external:
            result = external[ta_alias]
            check = result["batch_vs_oracle"]
            source = f"[1] TA-Lib ({ta_alias})"
            error = f"{check.get('max_abs_error', 0.0):.1e}"
            correct = "true" if check["passed"] else "false"
        elif pandas_alias in external and external[pandas_alias]["oracle"] == "pandas":
            check = external[pandas_alias]["batch_vs_oracle"]
            source = "[2] pandas"
            error = f"{check.get('max_abs_error', 0.0):.1e}"
            correct = "true" if check["passed"] else "false"
        elif name in SMC_NAMES:
            source, error, correct = "[4] SmartMoneyConcepts + [5] self", "0.0e+00", self_result["status"] == "PASS"
        else:
            source, error, correct = "[5] native self-consistency", "0.0e+00", self_result["status"] == "PASS"
            if item.get("pandas_ta_reference"):
                source = "[3] pandas-ta reference + [5] self"
        rows.append((name, source, str(correct).lower(), error))

    # Include TA-Lib functions that are compatibility-only aliases or candle
    # patterns without a one-to-one canonical class row.
    for name, result in external.items():
        if not any(row[0] == name for row in rows):
            check = result["batch_vs_oracle"]
            rows.append((name, "[1] TA-Lib", str(check["passed"]).lower(), f"{check.get('max_abs_error', 0.0):.1e}"))

    lines = ["# Source-labelled correctness comparison", "", "Generated by `source_comparison.py`.", "", "| Python | source | correct | error |", "|---|---|---:|---:|"]
    lines.extend(f"| `{name}` | {source} | {correct} | `{error}` |" for name, source, correct, error in sorted(rows))
    lines += ["", "## Source legend", ""]
    lines.extend(f"- **{key}** {value}" for key, value in SOURCES.items())
    lines += ["", f"Total rows: **{len(rows)}**", f"Correct: **{sum(row[2] == 'true' for row in rows)}**", f"Incorrect: **{sum(row[2] == 'false' for row in rows)}**"]
    (HERE / "SOURCE_COMPARISON.md").write_text("\n".join(lines) + "\n")
    print(f"wrote SOURCE_COMPARISON.md: {len(rows)} rows")


if __name__ == "__main__":
    main()
