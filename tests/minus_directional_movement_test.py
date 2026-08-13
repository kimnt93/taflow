import numpy as np
import talib

from taflow import MinusDirectionalMovement


def test_matches_talib_minus_dm() -> None:
    high = 100.0 + np.arange(128) + np.sin(np.arange(128) * 0.2)
    low = high - 2.0
    expected = talib.MINUS_DM(high, low, 14)
    actual = MinusDirectionalMovement(14).extend(high, low).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-12, equal_nan=True)
