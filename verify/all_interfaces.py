"""Self-consistency checks for every public taflow indicator adapter.

This complements ``verify.py``: TA-Lib functions use an external oracle there,
while this pass exercises the extended canonical API and checks that a full
constructor history is identical to a native state's ``extend`` history.
"""
from __future__ import annotations

import inspect
import json
from pathlib import Path

import numpy as np

import taflow
from taflow import executions

N = 128
close = np.linspace(100.0, 110.0, N)
high = close + 1.0
low = close - 1.0
open_ = close + 0.2
volume = np.linspace(100_000.0, 200_000.0, N)
benchmark = close * 1.01
periods = np.full(N, 10, dtype=np.int64)
condition = (np.arange(N) % 11) == 0
new_session = (np.arange(N) % 16) == 0
ARRAYS = {
    "_input": close, "input": close, "values": close, "price": close,
    "change": close, "value": close,
    "left": close, "right": benchmark, "x": close, "y": benchmark,
    "benchmark": benchmark, "close": close, "high": high, "low": low,
    "h": high, "l": low,
    "_open": open_, "open": open_, "volume": volume, "periods": periods,
    "condition": condition, "new_session": new_session, "anchor": new_session,
    "entry": condition, "_exit": ~condition,
    "input0": close, "input1": benchmark,
    "_input0": close, "_input1": benchmark,
}
SKIP = {
    "MaType", "ActiveZoneList",
}


def scalar(name: str, default: object) -> object:
    if default is not inspect.Parameter.empty:
        return default
    if name in {"quantile", "alpha"}:
        return 0.5
    if name == "percentile":
        return 50.0
    if name in {"gamma", "phase"}:
        return 0.5
    if name in {"stdev", "value_area"}:
        return 1.0
    if "average_type" in name or name == "matype":
        return 0
    if name in {"fastlimit"}:
        return 0.5
    if name in {"slowlimit"}:
        return 0.05
    if name in {"factor", "scalar", "multiplier"} or "factor" in name:
        return 0.7
    return 5


def kwargs_for(callable_: object, arrays: bool) -> dict[str, object]:
    result: dict[str, object] = {}
    for p in inspect.signature(callable_).parameters.values():
        if p.kind in (p.VAR_POSITIONAL, p.VAR_KEYWORD):
            continue
        if arrays and p.name in ARRAYS:
            result[p.name] = ARRAYS[p.name]
        elif p.default is inspect.Parameter.empty:
            result[p.name] = scalar(p.name, p.default)
    return result


def equal(left: object, right: object) -> bool:
    if isinstance(left, dict) or isinstance(right, dict):
        return isinstance(left, dict) and isinstance(right, dict) and left.keys() == right.keys() and all(equal(left[k], right[k]) for k in left)
    if isinstance(left, (tuple, list)) or isinstance(right, (tuple, list)):
        return isinstance(left, (tuple, list)) and isinstance(right, (tuple, list)) and len(left) == len(right) and all(equal(a, b) for a, b in zip(left, right))
    try:
        a, b = np.asarray(left, dtype=float), np.asarray(right, dtype=float)
        return a.shape == b.shape and np.allclose(a, b, equal_nan=True, rtol=1e-8, atol=1e-10)
    except (TypeError, ValueError):
        return left == right


def output(obj: object) -> object:
    if hasattr(obj, "compute"):
        return obj.compute()
    if hasattr(obj, "value"):
        return obj.value
    raise TypeError("adapter has neither compute() nor value")


def main() -> None:
    rows: list[dict[str, object]] = []
    # Public functional helpers are not stateful indicator classes, but still
    # belong to the all-interface correctness pass.
    helpers = {
        "RollingApply": lambda: executions.RollingApply(close, 5, lambda window: float(np.mean(window))),
        "SessionFlags": lambda: executions.SessionFlags(np.repeat(np.arange(8), N // 8)),
        "AdaptInput": lambda: executions.AdaptInput(close.tolist()),
        "AdaptOutput": lambda: executions.AdaptOutput(close, adapter="numpy"),
        "ToNumpy": lambda: executions.ToNumpy(close),
        "ToList": lambda: executions.ToList(close),
        "ToPandas": lambda: executions.ToPandas(close),
    }
    for name, check in helpers.items():
        try:
            result = np.asarray(check())
            if result.shape != (N,):
                raise AssertionError(f"unexpected output shape {result.shape}")
            rows.append({"name": name, "status": "PASS"})
        except Exception as exc:
            rows.append({"name": name, "status": "FAIL", "error": f"{type(exc).__name__}: {exc}"})
    for name in taflow.__all__:
        cls = getattr(taflow, name, None)
        if not isinstance(cls, type) or name.startswith("_") or name in SKIP:
            continue
        try:
            full = cls(**kwargs_for(cls, arrays=True))
            expected = output(full)
            if not hasattr(full, "extend"):
                raise TypeError("indicator has no extend()")
            state = cls(**kwargs_for(cls, arrays=False))
            state.extend(**kwargs_for(state.extend, arrays=True))
            actual = output(state)
            if not equal(expected, actual):
                raise AssertionError("constructor history differs from native extend history")
            live = cls(**kwargs_for(cls, arrays=False))
            append_params = tuple(inspect.signature(live.append).parameters.values())
            for index in range(N):
                row_kwargs = {}
                for parameter in append_params:
                    if parameter.kind in (parameter.VAR_POSITIONAL, parameter.VAR_KEYWORD):
                        continue
                    value = periods if parameter.name == "period" else ARRAYS.get(parameter.name)
                    row_kwargs[parameter.name] = value[index] if isinstance(value, np.ndarray) else scalar(parameter.name, parameter.default)
                live.append(**row_kwargs)
            if not equal(expected, output(live)):
                raise AssertionError("constructor history differs from one-bar append history")
            rows.append({"name": name, "status": "PASS"})
        except Exception as exc:  # report every interface; one failure must not hide others
            rows.append({"name": name, "status": "FAIL", "error": f"{type(exc).__name__}: {exc}"})
    out = Path(__file__).parent / "ALL_INTERFACES.json"
    out.write_text(json.dumps(rows, indent=2) + "\n")
    passed = sum(row["status"] == "PASS" for row in rows)
    failed = len(rows) - passed
    markdown = [
        "# Canonical taflow interface correctness",
        "",
        "Indicator oracle: constructor history == native `extend` history == one-bar `append` history; helper functions also receive smoke checks.",
        "",
        f"- Passed: **{passed}**",
        f"- Failed: **{failed}**",
        "",
        "| Interface | Status | Error |",
        "|---|---|---|",
    ]
    markdown.extend(f"| `{row['name']}` | {row['status']} | {row.get('error', '')} |" for row in rows)
    (Path(__file__).parent / "ALL_INTERFACES.md").write_text("\n".join(markdown) + "\n")
    print(f"{passed}/{len(rows)} public taflow interfaces passed; {failed} failed")
    if failed:
        for row in rows:
            if row["status"] == "FAIL":
                print(f"- {row['name']}: {row['error']}")
        raise SystemExit(1)


if __name__ == "__main__":
    main()
