import numpy as np

from taflow import Crossunder


def test_detects_crossunders_and_reset() -> None:
    left = np.array([2.0, 2.0, 1.0, 2.0])
    right = np.array([1.0, 1.0, 2.0, 1.0])
    indicator = Crossunder(left, right)
    np.testing.assert_array_equal(indicator.compute(), [0.0, 0.0, 1.0, 0.0])
    indicator.reset().append(1.0, 2.0)
    assert indicator.value == 0.0
