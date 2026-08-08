"""Executes the code examples embedded in README.md and docs/*.md.

Documentation that does not run is worse than none, so every Python fence in
the docs is executed. A document's fences run in order against one shared
namespace, the way a reader would follow it, so later blocks can use names
defined earlier.

Snippets that are deliberately illustrative rather than runnable are matched
by SKIP_SNIPPETS, each with the reason.
"""

import pathlib
import re

import numpy as np
import pytest

ROOT = pathlib.Path(__file__).resolve().parent.parent
DOCS = [ROOT / "README.md"] + sorted((ROOT / "docs").glob("*.md"))

SKIP_SNIPPETS = {
    "pip install": "shell",
    "make ": "shell",
    "git clone": "shell",
    "uv add": "shell",
    "maturin": "shell",
    "cargo": "shell",
    "SomeIndicator": "API summary with a placeholder class",
    "class MyAdapter": "illustrative adapter stub (MyContainer undefined)",
    "for tick in feed": "requires a live feed",
    "for tick in live_feed()": "requires a live feed",
    "feeds.items()": "requires a multi-symbol feed mapping",
    "per_symbol_arrays": "requires a multi-symbol feed mapping",
    'frame["date"]': "requires a session/date column",
    "atr.extend(high_history": "API summary, not runnable",
    "MyContainer": "illustrative adapter stub",
}


def base_namespace():
    """Fixture data under the names the documentation uses."""
    rng = np.random.default_rng(20260808)
    n = 500
    close = np.cumsum(rng.normal(0, 1, n)) + 100.0
    high = close + rng.random(n)
    low = close - rng.random(n)
    open_ = close + rng.normal(0, 0.2, n)
    volume = rng.random(n) * 1e6

    import pandas as pd

    import taflow
    from taflow import (
        AverageTrueRange,
        BollingerBands,
        ExponentialMovingAverage,
        SimpleMovingAverage,
        StochasticOscillator,
    )

    return {
        "np": np,
        "pd": pd,
        "taflow": taflow,
        "close": close,
        "high": high,
        "low": low,
        "open": open_,
        "o": open_,
        "h": high,
        "l": low,
        "c": close,
        "v": volume,
        "volume": volume,
        "history": close[:400],
        "next_close": float(close[400]),
        "price": close,
        "SimpleMovingAverage": SimpleMovingAverage,
        "ExponentialMovingAverage": ExponentialMovingAverage,
        "AverageTrueRange": AverageTrueRange,
        "BollingerBands": BollingerBands,
        "StochasticOscillator": StochasticOscillator,
        "frame": pd.DataFrame(
            {"open": open_, "high": high, "low": low, "close": close, "volume": volume}
        ),
        "sma": SimpleMovingAverage(close, timeperiod=10).compute(),
    }


def snippets(path):
    return re.findall(r"```python\n(.*?)```", path.read_text(), re.S)


def skip_reason(code):
    return next((why for frag, why in SKIP_SNIPPETS.items() if frag in code), None)


@pytest.mark.parametrize("path", DOCS, ids=lambda p: p.name)
def test_doc_examples_run(path):
    """Run one document's snippets in order against a shared namespace."""
    blocks = snippets(path)
    if not blocks:
        pytest.skip("no python examples")

    namespace = base_namespace()
    executed = 0
    for index, code in enumerate(blocks):
        if skip_reason(code):
            continue
        try:
            exec(compile(code, f"{path.name} block {index}", "exec"), namespace)
        except Exception as error:  # noqa: BLE001 - report which block broke
            pytest.fail(
                f"{path.name} block {index} raised "
                f"{type(error).__name__}: {error}\n\n{code}"
            )
        executed += 1

    assert executed, f"{path.name}: every python snippet was skipped"
