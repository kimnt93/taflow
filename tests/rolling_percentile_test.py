import numpy as np

from taflow import RollingPercentile


def test_rolling_percentile_lifecycle() -> None:
    values = np.array([4.0, 2.0, 3.0, 1.0, 5.0])
    indicator = RollingPercentile(values, timeperiod=3, percentile=50.0)
    expected = indicator.compute()
    indicator.reset().extend(values)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(values)
