import numpy as np
from taflow import PremiumDiscount


def test_premium_discount_lifecycle():
    close = np.linspace(90.0, 120.0, 64)
    indicator = PremiumDiscount(close, 20)
    assert len(indicator) == len(close)
    first = indicator.compute()
    indicator.reset().extend(close)
    for left, right in zip(first, indicator.compute()): np.testing.assert_array_equal(left, right)
