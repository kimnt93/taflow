import numpy as np

from taflow import CandleHaramiCross


def test_harami_cross_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleHaramiCross(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None

