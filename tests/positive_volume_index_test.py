import numpy as np
from taflow import PositiveVolumeIndex


def test_positive_volume_index_lifecycle() -> None:
    close = 100.0 + np.arange(32.0)
    volume = 1000.0 + np.arange(32.0) % 3
    state = PositiveVolumeIndex().extend(close, volume)
    first = state.compute()
    state.reset().extend(close, volume)
    np.testing.assert_array_equal(state.compute(), first)

