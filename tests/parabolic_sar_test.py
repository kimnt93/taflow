import numpy as np

from taflow import ParabolicSar


def test_parabolic_sar_lifecycle() -> None:
    high = np.linspace(101.0, 112.0, 12)
    low = high - 2.0
    indicator = ParabolicSar().extend(high, low)
    expected = indicator.compute()
    indicator.reset().extend(high, low)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(high)
