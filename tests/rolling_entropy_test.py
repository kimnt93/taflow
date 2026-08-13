import numpy as np

from taflow import RollingEntropy


def test_rolling_entropy_lifecycle_and_alignment():
    state = RollingEntropy(timeperiod=2)
    state.extend([1.0, 2.0])
    np.testing.assert_allclose(state.compute(), [np.nan, np.log(2.0)], equal_nan=True)
    assert len(state) == 2
    state.reset()
    assert state.value is None
