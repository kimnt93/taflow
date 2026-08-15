import numpy as np
import talib

from taflow import CandleClosingMarubozu


def test_closing_marubozu_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleClosingMarubozu().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    np.testing.assert_array_equal(indicator.compute(), talib.CDLCLOSINGMARUBOZU(values, values + 2.0, values - 2.0, values + 0.5))
    indicator.reset()
    assert indicator.value is None
