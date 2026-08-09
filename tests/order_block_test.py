import numpy as np

from taflow import OrderBlock


def test_order_block_lifecycle() -> None:
    close = 100.0 + np.sin(np.arange(128.0) / 7.0)
    high, low = close + 1.0, close - 1.0
    volume = np.arange(128.0) + 1000.0
    state = OrderBlock(high, low, close, volume, swing_length=5, internal_length=3, atr_period=14)
    first = state.compute()
    state.reset().extend(high, low, close, volume)
    for got, expected in zip(state.compute(), first):
        np.testing.assert_array_equal(got, expected)
    assert len(state) == len(close)

