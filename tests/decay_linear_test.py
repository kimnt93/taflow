import numpy as np
from taflow import DecayLinear


def test_decay_linear_lifecycle() -> None:
    values = np.arange(16.0)
    state = DecayLinear(values, timeperiod=4)
    first = state.compute()
    state.reset().extend(values)
    np.testing.assert_array_equal(state.compute(), first)

