import numpy as np
import talib

from taflow import MovingAverageConvergenceDivergenceFixed


def test_matches_talib_macdfix() -> None:
    close = 100.0 + np.sin(np.arange(300) * 0.11).cumsum()
    expected = talib.MACDFIX(close, 9)
    actual = MovingAverageConvergenceDivergenceFixed(close).compute()
    for got, want in zip(actual, expected):
        np.testing.assert_allclose(got, want, rtol=0.0, atol=2e-12, equal_nan=True)
