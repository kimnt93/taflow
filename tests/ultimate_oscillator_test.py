import numpy as np
import talib

from taflow import UltimateOscillator


def test_matches_talib_ultosc() -> None:
    index = np.arange(128, dtype=np.float64)
    high = 100.0 + index + np.sin(index * 0.2)
    low = high - 2.0
    close = high - 0.8
    expected = talib.ULTOSC(high, low, close, 7, 14, 28)
    actual = UltimateOscillator().extend(high, low, close).compute()
    np.testing.assert_allclose(actual, expected, equal_nan=True)
