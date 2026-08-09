import numpy as np
from taflow import TimeSeriesRank


def test_time_series_rank_lifecycle() -> None:
    values = np.arange(16.0)
    state = TimeSeriesRank(values, timeperiod=4)
    first = state.compute()
    state.reset().extend(values)
    np.testing.assert_array_equal(state.compute(), first)

