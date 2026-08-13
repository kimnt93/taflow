import numpy as np

from taflow import Liquidity


def test_liquidity_lifecycle() -> None:
    close = 100.0 + np.sin(np.arange(128.0) / 7.0)
    high, low = close + 1.0, close - 1.0
    state = Liquidity(swing_length=3, range_percent=0.01).extend(high, low)
    first = state.compute()
    state.reset().extend(high, low)
    for got, expected in zip(state.compute(), first):
        np.testing.assert_array_equal(got, expected)
    assert len(state) == len(close)

