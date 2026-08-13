import numpy as np

from taflow import RollingLinearRegressionAngle


def test_rolling_linear_regression_angle_lifecycle() -> None:
    indicator = RollingLinearRegressionAngle(timeperiod=3).extend(np.arange(8.0))
    assert len(indicator) == 8
    assert np.isnan(indicator.compute()[:2]).all()
    indicator.reset()
    assert len(indicator) == 0
