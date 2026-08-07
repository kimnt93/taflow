import numpy as np

from taflow import Cmf


def test_cmf_matches_reference_and_chunks():
    close = 100.0 + np.cumsum(np.sin(np.arange(100.0)) * 0.2)
    high = close + 1.0
    low = close - 1.0
    volume = np.arange(1.0, 101.0)
    period = 20
    expected = np.full(len(close), np.nan)
    multiplier = ((close - low) - (high - close)) / (high - low)
    for index in range(period - 1, len(close)):
        expected[index] = np.sum(multiplier[index - period + 1 : index + 1] * volume[index - period + 1 : index + 1]) / np.sum(volume[index - period + 1 : index + 1])
    full = Cmf(high=high, low=low, close=close, volume=volume, period=period).compute()
    np.testing.assert_allclose(full, expected, equal_nan=True, atol=1e-12)

    chunked = Cmf(period=period)
    for start in range(0, len(close), 13):
        chunked.extend(high[start : start + 13], low[start : start + 13], close[start : start + 13], volume[start : start + 13])
    np.testing.assert_array_equal(chunked.compute(), full)
