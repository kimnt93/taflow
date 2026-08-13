import numpy as np

from taflow import RollingCovariance


def test_rolling_cov_lifecycle() -> None:
    left = np.array([1.0, 4.0, 2.0, 8.0, 3.0])
    right = left * 2.0
    indicator = RollingCovariance(timeperiod=3).extend(left, right)
    expected = indicator.compute()
    indicator.reset().extend(left, right)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(left)
