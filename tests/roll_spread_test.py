import numpy as np
from taflow import RollSpread


def test_roll_spread_lifecycle():
    price = np.linspace(100.0, 120.0, 80)
    indicator = RollSpread(price, timeperiod=10)
    assert indicator.compute().shape == price.shape
    assert len(indicator) == len(price)
    indicator.reset().extend(price[:20])
    assert len(indicator) == 20
