import numpy as np
import talib

from taflow import WeightedMovingAverage


def test_matches_talib_wma() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(WeightedMovingAverage(10).extend(values).compute(), talib.WMA(values, 10), equal_nan=True)
