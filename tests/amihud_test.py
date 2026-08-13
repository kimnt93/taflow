import numpy as np
from taflow import Amihud


def test_amihud_lifecycle():
    close = np.linspace(100.0, 120.0, 80)
    volume = np.linspace(10.0, 20.0, 80)
    indicator = Amihud(timeperiod=10).extend(close, volume)
    assert indicator.compute().shape == close.shape
    assert len(indicator) == len(close)
    indicator.reset().extend(close[:20], volume[:20])
    assert len(indicator) == 20
