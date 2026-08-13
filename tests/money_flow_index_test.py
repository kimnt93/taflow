import numpy as np
import talib

from taflow import MoneyFlowIndex


def test_matches_talib_mfi() -> None:
    index = np.arange(128, dtype=np.float64)
    high = 100.0 + index + np.sin(index * 0.2)
    low = high - 2.0
    close = high - 0.8
    volume = 1000.0 + index * 3.0
    expected = talib.MFI(high, low, close, volume, 14)
    actual = MoneyFlowIndex(14).extend(high, low, close, volume).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-12, equal_nan=True)
