import numpy as np
import talib

from taflow import SimpleMovingAverage


def test_matches_talib_sma() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(SimpleMovingAverage(values, 10).compute(), talib.SMA(values, 10), equal_nan=True)
