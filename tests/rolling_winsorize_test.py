import numpy as np

from taflow import RollingWinsorize


def test_rolling_winsorize_lifecycle() -> None:
    values = np.array([4.0, 2.0, 3.0, 1.0, 5.0])
    indicator = RollingWinsorize(values, timeperiod=3, lower=0.0, upper=0.5)
    expected = indicator.compute()
    indicator.reset().extend(values)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(values)
