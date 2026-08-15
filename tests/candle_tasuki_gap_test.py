import numpy as np
import talib

from taflow import CandleTasukiGap


def test_tasuki_gap_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleTasukiGap().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    np.testing.assert_array_equal(indicator.compute(), talib.CDLTASUKIGAP(values, values + 2.0, values - 2.0, values + 0.5))
    indicator.reset()
    assert indicator.value is None
