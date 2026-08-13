import numpy as np
from taflow import OrnsteinUhlenbeckHalfLife


def test_ornstein_uhlenbeck_half_life_lifecycle():
    price = 100.0 + np.sin(np.linspace(0.0, 12.0, 80))
    indicator = OrnsteinUhlenbeckHalfLife(timeperiod=10).extend(price)
    assert indicator.compute().shape == price.shape
    assert len(indicator) == len(price)
    indicator.reset().extend(price[:20])
    assert len(indicator) == 20
