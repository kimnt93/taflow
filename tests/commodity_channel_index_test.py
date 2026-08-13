import numpy as np
import talib

from taflow import CommodityChannelIndex


def test_matches_talib_cci() -> None:
    index = np.arange(128, dtype=np.float64)
    high = 100.0 + index + np.sin(index * 0.2)
    low = high - 2.0
    close = high - 0.8
    expected = talib.CCI(high, low, close, 14)
    actual = CommodityChannelIndex(14).extend(high, low, close).compute()
    np.testing.assert_allclose(actual, expected, rtol=1e-8, atol=1e-10, equal_nan=True)
