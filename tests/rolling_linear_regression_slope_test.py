import numpy as np

from taflow import RollingLinearRegressionSlope


def test_rolling_linear_regression_slope_lifecycle() -> None:
    indicator = RollingLinearRegressionSlope(np.arange(8.0), timeperiod=3)
    assert len(indicator) == 8
    assert np.isnan(indicator.compute()[:2]).all()
