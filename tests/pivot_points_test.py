import numpy as np
from taflow import PivotPoints


def test_pivot_points_lifecycle():
    high = np.full(24, 110.0); low = np.full(24, 90.0); close = np.full(24, 100.0); anchor = np.array([True, False] * 12)
    indicator = PivotPoints(high, low, close, anchor); first = indicator.compute()
    indicator.reset().extend(high, low, close, anchor)
    for left, right in zip(first, indicator.compute()): np.testing.assert_array_equal(left, right)
