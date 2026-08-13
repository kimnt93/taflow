import numpy as np

from taflow import RollingLinearRegression


def test_rolling_linear_regression_lifecycle() -> None:
    values = np.arange(12.0)
    indicator = RollingLinearRegression(timeperiod=4).extend(values)
    assert len(indicator) == len(values)
    assert np.isnan(indicator.compute()[:3]).all()
    indicator.reset().extend(values[:4])
    assert len(indicator) == 4
