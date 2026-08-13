import numpy as np

from taflow import RollingQuantile


def test_rolling_quantile_lifecycle() -> None:
    values = np.array([4.0, 2.0, 3.0, 1.0, 5.0])
    indicator = RollingQuantile(timeperiod=3, quantile=0.5).extend(values)
    expected = indicator.compute()
    indicator.reset().extend(values)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(values)
