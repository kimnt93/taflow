import numpy as np
import talib

from taflow import CandleTwoCrows


def test_lifecycle_and_reset() -> None:
    values = np.arange(48.0) + 100.0
    indicator = CandleTwoCrows().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator.compute()) == len(values)
    np.testing.assert_array_equal(indicator.compute(), talib.CDL2CROWS(values, values + 2.0, values - 2.0, values + 0.5))
    assert indicator.reset().value is None
