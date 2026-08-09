import numpy as np

from taflow import RollingMinMaxIndex


def test_rolling_min_max_index_lifecycle() -> None:
    indicator = RollingMinMaxIndex(np.arange(8.0), timeperiod=3)
    minimum, maximum = indicator.compute()
    assert len(indicator) == 8
    assert np.all(minimum[:2] == 0)
    assert np.all(maximum[:2] == 0)
