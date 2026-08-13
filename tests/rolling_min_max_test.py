import numpy as np

from taflow import RollingMinMax


def test_rolling_min_max_lifecycle() -> None:
    indicator = RollingMinMax(timeperiod=3).extend(np.arange(8.0))
    minimum, maximum = indicator.compute()
    assert len(indicator) == 8
    assert np.isnan(minimum[:2]).all()
    assert np.isnan(maximum[:2]).all()
