import numpy as np

from taflow import CandleInvertedHammer


def test_inverted_hammer_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleInvertedHammer(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None

