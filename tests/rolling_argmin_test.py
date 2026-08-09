import numpy as np

from taflow import RollingArgmin


def test_rolling_argmin_lifecycle() -> None:
    values = np.array([4.0, 2.0, 3.0, 1.0, 5.0])
    indicator = RollingArgmin(values, timeperiod=3)
    expected = indicator.compute()
    indicator.reset().extend(values)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(values)
