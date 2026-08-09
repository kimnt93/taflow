import numpy as np
from taflow import Parkinson


def test_parkinson_lifecycle():
    high = np.linspace(101.0, 120.0, 80)
    low = high - 2.0
    indicator = Parkinson(high, low, timeperiod=10)
    assert indicator.compute().shape == high.shape
    assert len(indicator) == len(high)
    indicator.reset().extend(high[:20], low[:20])
    assert len(indicator) == 20
