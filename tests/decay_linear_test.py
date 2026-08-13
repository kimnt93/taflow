import numpy as np
from taflow import DecayLinear


def test_decay_linear_lifecycle() -> None:
    values = np.arange(16.0)
    state = DecayLinear(timeperiod=4).extend(values)
    first = state.compute()
    state.reset().extend(values)
    np.testing.assert_array_equal(state.compute(), first)

