import numpy as np

from taflow import Crossover


def test_detects_crossovers_and_reset() -> None:
    left = np.array([1.0, 1.0, 3.0, 2.0])
    right = np.array([2.0, 2.0, 2.0, 1.0])
    indicator = Crossover().extend(left, right)
    np.testing.assert_array_equal(indicator.compute(), [0.0, 0.0, 1.0, 0.0])
    indicator.reset().append(0.0, 1.0)
    assert indicator.value == 0.0
