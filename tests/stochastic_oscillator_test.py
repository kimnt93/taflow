import numpy as np
import talib

from taflow import StochasticOscillator


def test_matches_talib_stoch() -> None:
    index = np.arange(128, dtype=np.float64)
    high = 100.0 + index + np.sin(index * 0.2)
    low = high - 2.0
    close = high - 0.8
    expected = talib.STOCH(high, low, close, 5, 3, 0, 3, 0)
    actual = StochasticOscillator().extend(high, low, close).compute()
    for got, want in zip(actual, expected):
        np.testing.assert_allclose(got, want, equal_nan=True)
