import numpy as np

from taflow import RollingLinearRegressionIntercept


def test_rolling_linear_regression_intercept_lifecycle() -> None:
    indicator = RollingLinearRegressionIntercept(np.arange(8.0), timeperiod=3)
    assert len(indicator) == 8
    assert np.isnan(indicator.compute()[:2]).all()
