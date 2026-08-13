import numpy as np
import talib

from taflow import StochasticRelativeStrengthIndex


def test_matches_talib_stochrsi() -> None:
    values = 100.0 + np.arange(160) * 0.1 + np.sin(np.arange(160) * 0.2)
    expected = talib.STOCHRSI(values, 14, 5, 3, 0)
    actual = StochasticRelativeStrengthIndex().extend(values).compute()
    for got, want in zip(actual, expected):
        np.testing.assert_allclose(got, want, equal_nan=True)
