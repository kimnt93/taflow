import numpy as np
import talib

from taflow import CandleEveningStar


def test_evening_star_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleEveningStar().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None


def test_evening_star_matches_talib_default_penetration_and_chunks() -> None:
    rng = np.random.default_rng(751)
    close = 100.0 + rng.normal(0.0, 0.6, 263).cumsum()
    open_ = close + rng.normal(0.0, 0.5, 263)
    high = np.maximum(open_, close) + rng.uniform(0.1, 1.0, 263)
    low = np.minimum(open_, close) - rng.uniform(0.1, 1.0, 263)
    expected = talib.CDLEVENINGSTAR(open_, high, low, close, penetration=0.3)
    all_at_once = CandleEveningStar().extend(open_, high, low, close)
    np.testing.assert_array_equal(all_at_once.compute(), expected)
    chunked = CandleEveningStar()
    for start in range(0, len(open_), 13):
        chunked.extend(open_[start : start + 13], high[start : start + 13], low[start : start + 13], close[start : start + 13])
    np.testing.assert_array_equal(chunked.compute(), expected)
    assert chunked.value == all_at_once.value
