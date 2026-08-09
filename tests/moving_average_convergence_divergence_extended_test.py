import numpy as np
import talib

from taflow import MovingAverageConvergenceDivergenceExtended


def test_matches_talib_macdext_default_ema() -> None:
    close = 100.0 + np.sin(np.arange(300) * 0.17).cumsum()
    expected = talib.MACDEXT(close, 12, 1, 26, 1, 9, 1)
    actual = MovingAverageConvergenceDivergenceExtended(close).compute()
    for got, want in zip(actual, expected):
        np.testing.assert_allclose(got, want, rtol=0.0, atol=2e-12, equal_nan=True)
