import numpy as np
from taflow import SignedPower


def test_signed_power_lifecycle() -> None:
    values = np.array([-2.0, 0.0, 3.0])
    state = SignedPower().extend(values)
    np.testing.assert_array_equal(state.compute(), [-4.0, 0.0, 9.0])
    state.reset().extend(values)
    np.testing.assert_array_equal(state.compute(), [-4.0, 0.0, 9.0])

