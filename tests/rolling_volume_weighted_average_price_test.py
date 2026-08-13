import numpy as np

from taflow import RollingVolumeWeightedAveragePrice


def test_rolling_vwap_lifecycle_and_reset():
    state = RollingVolumeWeightedAveragePrice(timeperiod=2)
    state.extend([11.0, 12.0], [9.0, 10.0], [10.0, 11.0], [2.0, 2.0])
    np.testing.assert_allclose(state.compute(), [np.nan, 10.5], equal_nan=True)
    assert len(state) == 2
    state.reset()
    assert state.value is None
