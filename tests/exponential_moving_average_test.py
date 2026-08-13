import numpy as np
import talib

from taflow import ExponentialMovingAverage


def test_matches_talib_ema() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(ExponentialMovingAverage(10).extend(values).compute(), talib.EMA(values, 10), equal_nan=True)
