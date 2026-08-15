import numpy as np
import talib

from taflow import CandleMorningDojiStar


def test_morning_doji_star_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleMorningDojiStar().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    np.testing.assert_array_equal(
        indicator.compute(),
        talib.CDLMORNINGDOJISTAR(values, values + 2.0, values - 2.0, values + 0.5),
    )
    indicator.reset()
    assert indicator.value is None
