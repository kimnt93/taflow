import numpy as np

from taflow import Cross


def test_detects_upward_and_downward_crossings() -> None:
    indicator = Cross(np.array([0.0, 2.0, 0.0]), np.array([1.0, 1.0, 1.0]))
    np.testing.assert_array_equal(indicator.compute(), [0.0, 1.0, 1.0])
    assert indicator.reset().value is None
