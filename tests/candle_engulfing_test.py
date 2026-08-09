import numpy as np

from taflow import CandleEngulfing


def test_engulfing_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleEngulfing(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None

