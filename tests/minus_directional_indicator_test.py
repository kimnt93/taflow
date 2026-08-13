import numpy as np
import talib

from taflow import MinusDirectionalIndicator


def test_matches_talib_minus_di() -> None:
    high = 100.0 + np.arange(128) + np.sin(np.arange(128) * 0.2)
    low = high - 2.0
    close = high - 0.8
    expected = talib.MINUS_DI(high, low, close, 14)
    actual = MinusDirectionalIndicator(14).extend(high, low, close).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-12, equal_nan=True)
