#!/usr/bin/env python3
"""Regenerate docs/INDICATORS.md from the installed package.

Signatures are introspected from the live classes so the reference cannot
drift from the implementation. TA-Lib names and input lists come from the
verification harness's benchmark metadata.

    python scripts/gen_indicators_doc.py
"""

import inspect
import pathlib
import re

from verification.registry import build_registry

ROOT = pathlib.Path(__file__).resolve().parent.parent
SERIES = {
    "_input",
    "values",
    "price",
    "close",
    "high",
    "low",
    "open",
    "_open",
    "volume",
    "real",
    "input",
    "series",
    "x",
    "y",
    "periods",
    "condition",
    "new_session",
    "session_id",
}

CATEGORIES = [
    "Moving averages & overlap",
    "Momentum & trend",
    "Volatility & bands",
    "Volume",
    "Price transforms",
    "Rolling & statistical operators",
    "Cycle (Hilbert transform)",
    "Math transforms",
    "Candlestick patterns",
    "Market structure & sessions",
    "Quant & econometrics",
    "Signal & series operators",
]


def load_metadata():
    """Return canonical indicator metadata from the verification registry."""
    return {
        spec.cls.__name__: {
            "class": spec.cls,
            "talib": spec.talib_name,
            "inputs": list(spec.series_args),
        }
        for spec in build_registry().values()
    }


def signature(cls, inputs):
    try:
        params = list(inspect.signature(cls.__init__).parameters.values())[1:]
    except (TypeError, ValueError):
        return None
    cfg = (
        ", ".join(
            f"{p.name}={p.default!r}"
            if p.default is not inspect.Parameter.empty
            else p.name
            for p in params
        )
        or "—"
    )
    return {
        "cfg": cfg,
        "inputs": ", ".join(inputs) or "—",
    }


def category(name, talib):
    checks = [
        ("Candlestick patterns", name.startswith("Candle")),
        (
            "Math transforms",
            name.startswith("Math") or talib in {"ADD", "SUB", "MULT", "DIV"},
        ),
        ("Cycle (Hilbert transform)", name.startswith("HilbertTransform")),
        (
            "Rolling & statistical operators",
            name.startswith("Rolling") or name.startswith("ExponentiallyWeighted"),
        ),
        (
            "Moving averages & overlap",
            any(
                k in name
                for k in (
                    "MovingAverage",
                    "Ema",
                    "Sma",
                    "McGinley",
                    "Vidya",
                    "Jurik",
                    "Trima",
                    "Kama",
                    "Mama",
                    "TripleExponential",
                    "DoubleExponential",
                    "ZeroLag",
                    "Arnaud",
                    "Hull",
                )
            ),
        ),
        (
            "Volume",
            any(
                k in name
                for k in (
                    "Volume",
                    "Obv",
                    "Amihud",
                    "Klinger",
                    "Accumulation",
                    "Force",
                    "EaseOfMovement",
                    "ChaikinMoneyFlow",
                    "MoneyFlow",
                )
            ),
        ),
        (
            "Volatility & bands",
            any(
                k in name
                for k in (
                    "TrueRange",
                    "Volatility",
                    "Bands",
                    "Keltner",
                    "Donchian",
                    "Ulcer",
                    "Parkinson",
                    "GarmanKlass",
                    "RogersSatchell",
                    "YangZhang",
                    "Squeeze",
                    "Supertrend",
                    "CloseToCloseSigma",
                )
            ),
        ),
        (
            "Market structure & sessions",
            any(
                k in name
                for k in (
                    "OrderBlock",
                    "Liquidity",
                    "FairValueGap",
                    "Swing",
                    "BreakOfStructure",
                    "PremiumDiscount",
                    "EqualHighs",
                    "Retracement",
                    "Fibonacci",
                    "PreviousHighLow",
                    "InsideBar",
                    "OutsideBar",
                    "HigherHigh",
                    "LowerLow",
                    "GapUp",
                    "GapDown",
                    "OpeningRange",
                    "Session",
                    "PivotPoints",
                )
            ),
        ),
        (
            "Quant & econometrics",
            any(
                k in name
                for k in (
                    "Kalman",
                    "Ornstein",
                    "SpreadZScore",
                    "FracDiff",
                    "RollSpread",
                    "Hurst",
                    "FractalDimension",
                    "HedgeRatio",
                    "CumulativeSumControlChart",
                )
            ),
        ),
        (
            "Signal & series operators",
            any(
                k in name
                for k in (
                    "Cumulative",
                    "Crossover",
                    "Crossunder",
                    "Rising",
                    "Falling",
                    "BarsSince",
                    "ValueWhen",
                    "Lag",
                    "SignalDelay",
                    "SignedPower",
                    "TimeSeriesRank",
                    "Drawdown",
                    "HighestSince",
                    "LowestSince",
                    "DecayLinear",
                    "LogReturn",
                    "EntryExit",
                    "PositionHold",
                )
            ),
        ),
        ("Price transforms", any(k in name for k in ("Price", "HeikinAshi"))),
    ]
    for label, hit in checks:
        if hit:
            return label
    return "Momentum & trend"


def main():
    meta = load_metadata()
    rows = []
    for name in sorted(meta):
        cls = meta[name]["class"]
        sig = signature(cls, meta[name]["inputs"])
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

ind = SimpleMovingAverage(timeperiod=30)         # configuration only
ind.extend(close)                                # historical backfill
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
        "### Input and configuration order\n",
        """Every constructor accepts configuration only. Historical series are
passed to ``extend`` in the documented input order. Configuration values have
defaults unless the algorithm cannot define one semantically:

```python
from taflow import SimpleMovingAverage, MoneyFlowIndex

SimpleMovingAverage(timeperiod=30).extend(close)
MoneyFlowIndex(timeperiod=14).extend(high, low, close, volume)
```

The `Input order` and `Constructor configuration` columns below are
authoritative: they are introspected from the live ``extend`` and constructor
signatures. Passing data by keyword always works.
""",
        "Correctness is reported in [../verify/CORRECTNESS.md](../verify/CORRECTNESS.md); "
        "throughput is in [../verify/BENCHMARK.md](../verify/BENCHMARK.md).\n",
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
        out.append("| Class | TA-Lib | Input order | Constructor configuration |")
        out.append("|---|---|---|---|")
        for row in sorted(grouped[cat], key=lambda r: r["cls"]):
            out.append(
                f"| `{row['cls']}` | {row['talib'] or '—'} | `({row['inputs']})` "
                f"| `({row['cfg']})` |"
            )
        out.append("")

    target = ROOT / "docs/INDICATORS.md"
    target.write_text("\n".join(out))
    print(f"wrote {target.relative_to(ROOT)}: {len(rows)} classes, {talib_n} TA-Lib")


if __name__ == "__main__":
    main()
