#!/usr/bin/env python3
"""Regenerate docs/INDICATORS.md from the installed package.

Signatures are introspected from the live classes so the reference cannot
drift from the implementation. TA-Lib names and input lists come from the
verification harness's benchmark metadata.

    python scripts/gen_indicators_doc.py
"""

import glob
import inspect
import json
import pathlib
import re

import taflow

ROOT = pathlib.Path(__file__).resolve().parent.parent
SERIES = {"_input", "values", "price", "close", "high", "low", "open", "_open",
          "volume", "real", "input", "series", "x", "y", "periods", "condition",
          "new_session", "session_id"}

CATEGORIES = [
    "Moving averages & overlap", "Momentum & trend", "Volatility & bands",
    "Volume", "Price transforms", "Rolling & statistical operators",
    "Cycle (Hilbert transform)", "Math transforms", "Candlestick patterns",
    "Market structure & sessions", "Quant & econometrics",
    "Signal & series operators",
]


def load_metadata():
    meta = {}
    for path in glob.glob(str(ROOT / "verify/benchmark_reports/*.json")):
        with open(path) as handle:
            data = json.load(handle)
        meta[data["canonical_class"]] = {"talib": data.get("talib_name"),
                                         "inputs": data.get("inputs", [])}
    return meta


def signature(cls):
    try:
        params = list(inspect.signature(cls.__init__).parameters.values())[1:]
    except (TypeError, ValueError):
        return None
    config = [p for p in params if p.name not in SERIES]
    cfg = ", ".join(
        f"{p.name}={p.default!r}" if p.default is not inspect.Parameter.empty else p.name
        for p in config) or "—"
    return {"cfg": cfg,
            "order": ", ".join(p.name for p in params),
            "series_first": bool(params) and params[0].name in SERIES}


def category(name, talib):
    checks = [
        ("Candlestick patterns", name.startswith("Candle")),
        ("Math transforms", name.startswith("Math") or talib in {"ADD", "SUB", "MULT", "DIV"}),
        ("Cycle (Hilbert transform)", name.startswith("HilbertTransform")),
        ("Rolling & statistical operators",
         name.startswith("Rolling") or name.startswith("ExponentiallyWeighted")),
        ("Moving averages & overlap", any(k in name for k in (
            "MovingAverage", "Ema", "Sma", "McGinley", "Vidya", "Jurik", "Trima",
            "Kama", "Mama", "TripleExponential", "DoubleExponential", "ZeroLag",
            "Arnaud", "Hull"))),
        ("Volume", any(k in name for k in (
            "Volume", "Obv", "Amihud", "Klinger", "Accumulation", "Force",
            "EaseOfMovement", "ChaikinMoneyFlow", "MoneyFlow"))),
        ("Volatility & bands", any(k in name for k in (
            "TrueRange", "Volatility", "Bands", "Keltner", "Donchian", "Ulcer",
            "Parkinson", "GarmanKlass", "RogersSatchell", "YangZhang", "Squeeze",
            "Supertrend", "CloseToCloseSigma"))),
        ("Market structure & sessions", any(k in name for k in (
            "OrderBlock", "Liquidity", "FairValueGap", "Swing", "BreakOfStructure",
            "PremiumDiscount", "EqualHighs", "Retracement", "Fibonacci",
            "PreviousHighLow", "InsideBar", "OutsideBar", "HigherHigh", "LowerLow",
            "GapUp", "GapDown", "OpeningRange", "Session", "PivotPoints"))),
        ("Quant & econometrics", any(k in name for k in (
            "Kalman", "Ornstein", "SpreadZScore", "FracDiff", "RollSpread", "Hurst",
            "FractalDimension", "HedgeRatio", "CumulativeSumControlChart"))),
        ("Signal & series operators", any(k in name for k in (
            "Cumulative", "Crossover", "Crossunder", "Rising", "Falling", "BarsSince",
            "ValueWhen", "Lag", "SignalDelay", "SignedPower", "TimeSeriesRank",
            "Drawdown", "HighestSince", "LowestSince", "DecayLinear", "LogReturn",
            "EntryExit", "PositionHold"))),
        ("Price transforms", any(k in name for k in ("Price", "HeikinAshi"))),
    ]
    for label, hit in checks:
        if hit:
            return label
    return "Momentum & trend"


def main():
    meta = load_metadata()
    rows = []
    for name in sorted(set(dir(taflow)) | set(meta)):
        cls = getattr(taflow, name, None)
        if not inspect.isclass(cls) or name.startswith("_"):
            continue
        sig = signature(cls)
        if sig is None:
            continue
        entry = dict(sig)
        entry["cls"] = name
        entry["talib"] = meta.get(name, {}).get("talib")
        entry["cat"] = category(name, entry["talib"] or "")
        rows.append(entry)

    grouped = {}
    for row in rows:
        grouped.setdefault(row["cat"], []).append(row)
    talib_n = sum(1 for r in rows if r["talib"])

    out = [
        "# TAFlow indicator reference\n",
        f"**{len(rows)}** classes — **{talib_n}** with a TA-Lib equivalent, "
        f"**{len(rows) - talib_n}** extended operators with no TA-Lib counterpart.\n",
        "> Generated by `scripts/gen_indicators_doc.py` from the installed package. "
        "Do not edit by hand.\n",
        "## The shared contract\n",
        """Every class behaves the same way:

```python
from taflow import SimpleMovingAverage

ind = SimpleMovingAverage(close, timeperiod=30)  # data may go in the constructor
ind.append(float(close[-1]))                     # O(1) live update
ind.value                                        # latest value, None during warm-up
ind.compute()                                    # full aligned series
len(ind)                                         # bars consumed
ind.reset()                                      # clear state and history in place
```

Outputs are `float64` arrays the same length as the input, `NaN` through
warm-up. Multi-output indicators return a tuple. Candle patterns return
`int32` scores (`0`, `±100`).
""",
        "### Argument order\n",
        """Every stateful indicator takes its required input series **first**, then
configuration. Configuration values have defaults unless the algorithm cannot
define one semantically:

```python
from taflow import SimpleMovingAverage, MoneyFlowIndex

SimpleMovingAverage(close, timeperiod=30)
MoneyFlowIndex(high, low, close, volume, timeperiod=14)
```

The `Constructor order` column below is authoritative — it is introspected from
the live signature. Passing data by keyword always works.
""",
        "Correctness uses the highest-priority available external oracle in "
        "[../verify/SOURCE_COMPARISON.md](../verify/SOURCE_COMPARISON.md); throughput is in "
        "[../verify/benchmark_reports/BENCHMARK.md](../verify/benchmark_reports/BENCHMARK.md).\n",
        "## Contents\n",
    ]
    for cat in CATEGORIES:
        if cat in grouped:
            anchor = re.sub(r"[^a-z0-9 -]", "", cat.lower()).replace(" ", "-")
            out.append(f"- [{cat}](#{anchor}) — {len(grouped[cat])}")
    out.append("")
    for cat in CATEGORIES:
        if cat not in grouped:
            continue
        out.append(f"## {cat}\n")
        out.append("| Class | TA-Lib | Parameters | Constructor order |")
        out.append("|---|---|---|---|")
        for row in sorted(grouped[cat], key=lambda r: r["cls"]):
            out.append(f"| `{row['cls']}` | {row['talib'] or '—'} | {row['cfg']} "
                       f"| `({row['order']})` |")
        out.append("")

    target = ROOT / "docs/INDICATORS.md"
    target.write_text("\n".join(out))
    print(f"wrote {target.relative_to(ROOT)}: {len(rows)} classes, "
          f"{talib_n} TA-Lib")


if __name__ == "__main__":
    main()
