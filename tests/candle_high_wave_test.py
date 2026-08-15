import numpy as np
import talib

from taflow import CandleHighWave


def test_high_wave_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleHighWave().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None


def test_high_wave_matches_talib_and_chunk_lifecycle() -> None:
    rng = np.random.default_rng(743)
    close = 100.0 + rng.normal(0.0, 0.5, 257).cumsum()
    open_ = close + rng.normal(0.0, 0.4, 257)
    high = np.maximum(open_, close) + rng.uniform(0.1, 1.2, 257)
    low = np.minimum(open_, close) - rng.uniform(0.1, 1.2, 257)
    expected = talib.CDLHIGHWAVE(open_, high, low, close)
    all_at_once = CandleHighWave().extend(open_, high, low, close)
    np.testing.assert_array_equal(all_at_once.compute(), expected)
    chunked = CandleHighWave()
    for start in range(0, len(open_), 11):
        chunked.extend(open_[start : start + 11], high[start : start + 11], low[start : start + 11], close[start : start + 11])
    np.testing.assert_array_equal(chunked.compute(), expected)
    assert chunked.value == all_at_once.value
