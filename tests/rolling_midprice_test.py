import numpy as np
import talib

from taflow import RollingMidprice


def test_matches_talib_midprice() -> None:
    index = np.arange(128, dtype=np.float64)
    high = 100.0 + index + np.sin(index * 0.17)
    low = high - 2.0
    np.testing.assert_allclose(RollingMidprice(10).extend(high, low).compute(), talib.MIDPRICE(high, low, 10), equal_nan=True)
