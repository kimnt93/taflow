import numpy as np
import talib

from taflow import FastStochasticOscillator


def test_matches_talib_stochf() -> None:
    index = np.arange(128, dtype=np.float64)
    high = 100.0 + index + np.sin(index * 0.2)
    low = high - 2.0
    close = high - 0.8
    expected = talib.STOCHF(high, low, close, 5, 3, 0)
    actual = FastStochasticOscillator(high, low, close).compute()
    for got, want in zip(actual, expected):
        np.testing.assert_allclose(got, want, equal_nan=True)
