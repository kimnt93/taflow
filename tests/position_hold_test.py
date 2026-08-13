import numpy as np

from taflow import PositionHold


def test_holds_latest_non_zero_position() -> None:
    indicator = PositionHold().extend(np.array([0.0, 2.0, 0.0, -1.0, 0.0]))
    np.testing.assert_array_equal(indicator.compute(), [0.0, 2.0, 2.0, -1.0, -1.0])
    assert indicator.reset().value is None
