import numpy as np
import talib

from taflow import KaufmanAdaptiveMovingAverage


def test_matches_talib_kama() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(KaufmanAdaptiveMovingAverage(values, 10).compute(), talib.KAMA(values, 10), equal_nan=True)
