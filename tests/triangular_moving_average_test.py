import numpy as np
import talib

from taflow import TriangularMovingAverage


def test_matches_talib_trima() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(TriangularMovingAverage(values, 10).compute(), talib.TRIMA(values, 10), equal_nan=True)
