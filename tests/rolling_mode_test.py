import numpy as np

from taflow import RollingMode


def test_rolling_mode_lifecycle() -> None:
    values = np.array([4.0, 2.0, 2.0, 1.0, 5.0])
    indicator = RollingMode(timeperiod=3).extend(values)
    expected = indicator.compute()
    indicator.reset().extend(values)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(values)
