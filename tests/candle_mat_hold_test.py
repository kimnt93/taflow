import numpy as np
import talib

from taflow import CandleMatHold


def test_mat_hold_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleMatHold().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None


def test_mat_hold_matches_talib_default_penetration_and_chunks() -> None:
    rng = np.random.default_rng(757)
    close = 100.0 + rng.normal(0.0, 0.6, 269).cumsum()
    open_ = close + rng.normal(0.0, 0.5, 269)
    high = np.maximum(open_, close) + rng.uniform(0.1, 1.0, 269)
    low = np.minimum(open_, close) - rng.uniform(0.1, 1.0, 269)
    expected = talib.CDLMATHOLD(open_, high, low, close, penetration=0.5)
    all_at_once = CandleMatHold().extend(open_, high, low, close)
    np.testing.assert_array_equal(all_at_once.compute(), expected)
    chunked = CandleMatHold()
    for start in range(0, len(open_), 15):
        chunked.extend(open_[start : start + 15], high[start : start + 15], low[start : start + 15], close[start : start + 15])
    np.testing.assert_array_equal(chunked.compute(), expected)
    assert chunked.value == all_at_once.value
