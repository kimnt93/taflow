import numpy as np

from taflow import AverageDailyDollarValue


def test_average_daily_dollar_value_lifecycle():
    close = np.linspace(100.0, 120.0, 80)
    volume = np.linspace(10.0, 20.0, 80)
    indicator = AverageDailyDollarValue(timeperiod=10).extend(close, volume)
    assert indicator.compute().shape == close.shape
    assert len(indicator) == len(close)
    indicator.reset().extend(close[:20], volume[:20])
    assert len(indicator) == 20
