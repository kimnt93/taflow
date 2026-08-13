import numpy as np
from taflow import OpeningRange


def test_opening_range_lifecycle():
    close = np.linspace(90.0, 120.0, 32); high = close + 1.0; low = close - 1.0; anchor = np.zeros(len(close), dtype=bool)
    indicator = OpeningRange(5).extend(high, low, close, anchor); first = indicator.compute()
    indicator.reset().extend(high, low, close, anchor)
    for left, right in zip(first, indicator.compute()): np.testing.assert_array_equal(left, right)
