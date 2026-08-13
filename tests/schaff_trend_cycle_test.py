import numpy as np

from taflow import SchaffTrendCycle


def test_schaff_trend_cycle_lifecycle():
    close = np.linspace(100.0, 120.0, 80)
    indicator = SchaffTrendCycle().extend(close)
    outputs = indicator.compute()
    assert all(array.shape == close.shape for array in outputs)
    assert len(indicator) == len(close)
    indicator.reset().extend(close[:20])
    assert len(indicator) == 20
