import numpy as np
import talib

from taflow import RollingAverageDeviation


def test_matches_talib_avgdev() -> None:
    cases = (
        np.sin(np.arange(128, dtype=np.float64) * 0.17),
        np.full(128, 7.25),
        np.arange(128, dtype=np.float64) * 0.125,
    )
    for period in (2, 10, 30):
        for values in cases:
            expected = talib.AVGDEV(values, timeperiod=period)
            actual = RollingAverageDeviation(period).extend(values).compute()
            np.testing.assert_allclose(actual, expected, rtol=0.0, atol=0.0, equal_nan=True)
