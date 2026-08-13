import numpy as np
import talib

from taflow import BollingerBands


def test_matches_talib_bbands() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    expected = talib.BBANDS(values, 20, 2.0, 2.0, 0)
    actual = BollingerBands(20).extend(values).compute()
    for got, want in zip(actual, expected):
        np.testing.assert_allclose(got, want, equal_nan=True)
