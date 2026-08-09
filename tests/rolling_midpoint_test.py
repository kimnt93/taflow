import numpy as np
import talib

from taflow import RollingMidpoint


def test_matches_talib_midpoint() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(RollingMidpoint(values, 10).compute(), talib.MIDPOINT(values, 10), equal_nan=True)
