import numpy as np

from taflow import CandleClosingMarubozu


def test_closing_marubozu_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleClosingMarubozu(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None

