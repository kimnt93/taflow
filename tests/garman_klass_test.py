import numpy as np
from taflow import GarmanKlass


def test_garman_klass_lifecycle():
    open_ = np.linspace(100.0, 120.0, 80)
    high = open_ + 1.0
    low = open_ - 1.0
    close = open_ + 0.5
    indicator = GarmanKlass(open_, high, low, close, timeperiod=10)
    assert indicator.compute().shape == open_.shape
    assert len(indicator) == len(open_)
    indicator.reset().extend(open_[:20], high[:20], low[:20], close[:20])
    assert len(indicator) == 20
