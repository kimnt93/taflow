import numpy as np
from taflow import ChaikinMoneyFlow


def test_chaikin_money_flow_lifecycle() -> None:
    close = 100.0 + np.arange(64.0)
    high, low, volume = close + 1.0, close - 1.0, np.full(64, 1000.0)
    state = ChaikinMoneyFlow(period=5).extend(high, low, close, volume)
    first = state.compute()
    state.reset().extend(high, low, close, volume)
    np.testing.assert_array_equal(state.compute(), first)

