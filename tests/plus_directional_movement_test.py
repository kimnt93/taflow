import numpy as np

from taflow import PlusDirectionalMovement


def test_plus_directional_movement_lifecycle() -> None:
    high = np.linspace(101.0, 112.0, 12)
    low = high - 2.0
    indicator = PlusDirectionalMovement(timeperiod=3).extend(high, low)
    expected = indicator.compute()
    indicator.reset().extend(high, low)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(high)
