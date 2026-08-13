import numpy as np
from taflow import SessionVolumeLevels


def test_session_volume_levels_lifecycle():
    high = np.full(32, 110.0); low = np.full(32, 90.0); close = np.full(32, 100.0); volume = np.full(32, 1000.0); anchor = np.zeros(32, dtype=bool)
    indicator = SessionVolumeLevels().extend(high, low, close, volume, anchor); first = indicator.compute()
    indicator.reset().extend(high, low, close, volume, anchor)
    for left, right in zip(first, indicator.compute()): np.testing.assert_array_equal(left, right)
