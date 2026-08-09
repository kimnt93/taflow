import numpy as np
from taflow import NegativeVolumeIndex


def test_negative_volume_index_lifecycle() -> None:
    close = 100.0 + np.arange(32.0)
    volume = 1000.0 + np.arange(32.0) % 3
    state = NegativeVolumeIndex(close, volume)
    first = state.compute()
    state.reset().extend(close, volume)
    np.testing.assert_array_equal(state.compute(), first)

