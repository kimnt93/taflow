import numpy as np

from taflow import SpreadZScore


def test_spread_zscore_lifecycle():
    x = np.linspace(10.0, 20.0, 80)
    y = x * 2.0
    indicator = SpreadZScore(x, y, timeperiod=10)
    assert indicator.compute().shape == x.shape
    assert len(indicator) == len(x)
    indicator.reset().extend(x[:20], y[:20])
    assert len(indicator) == 20
