import numpy as np
import talib

from taflow import DoubleExponentialMovingAverage


def test_matches_talib_dema() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(DoubleExponentialMovingAverage(10).extend(values).compute(), talib.DEMA(values, 10), equal_nan=True)
