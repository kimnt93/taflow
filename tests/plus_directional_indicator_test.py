import numpy as np

from taflow import PlusDirectionalIndicator


def test_plus_directional_indicator_lifecycle() -> None:
    close = np.linspace(100.0, 112.0, 12)
    high = close + 1.0
    low = close - 1.0
    indicator = PlusDirectionalIndicator(timeperiod=3).extend(high, low, close)
    expected = indicator.compute()
    indicator.reset().extend(high, low, close)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(close)
