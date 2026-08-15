import numpy as np
import talib

from taflow import BollingerBands


def test_matches_talib_bbands_for_every_moving_average_type() -> None:
    values = 100.0 + np.arange(256) * 0.2 + np.sin(np.arange(256) * 0.17)
    for moving_average_type in range(9):
        expected = talib.BBANDS(values, 20, 2.3, 1.7, moving_average_type)
        actual = BollingerBands(20, 2.3, 1.7, moving_average_type).extend(values).compute()
        for got, want in zip(actual, expected):
            np.testing.assert_allclose(got, want, rtol=1e-10, atol=1e-10, equal_nan=True)
