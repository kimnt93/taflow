import numpy as np

from taflow import RollingTimeSeriesForecast


def test_rolling_time_series_forecast_lifecycle() -> None:
    indicator = RollingTimeSeriesForecast(timeperiod=3).extend(np.arange(8.0))
    assert len(indicator) == 8
    assert np.isnan(indicator.compute()[:2]).all()
