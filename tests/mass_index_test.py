import numpy as np
from taflow import MassIndex


def test_mass_index_lifecycle() -> None:
    high = 100.0 + np.arange(64.0)
    low = high - 2.0
    state = MassIndex(high, low, ema_period=3, sum_period=5)
    first = state.compute()
    state.reset().extend(high, low)
    np.testing.assert_array_equal(state.compute(), first)
    assert len(state) == len(high)

