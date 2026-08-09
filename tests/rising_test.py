import numpy as np

from taflow import Rising


def test_rising_lifecycle_and_reset():
    state = Rising(np.array([], dtype=float), timeperiod=2)
    state.extend([1.0, 2.0, 3.0])
    np.testing.assert_allclose(state.compute(), [np.nan, np.nan, 1.0], equal_nan=True)
    assert len(state) == 3
    state.reset().append(1.0)
    assert state.value is None
