import numpy as np

from taflow import Falling


def test_falling_lifecycle_and_reset():
    state = Falling(timeperiod=2)
    state.extend([3.0, 2.0, 1.0])
    np.testing.assert_allclose(state.compute(), [np.nan, np.nan, 1.0], equal_nan=True)
    assert len(state) == 3
    state.reset().append(3.0)
    assert state.value is None
