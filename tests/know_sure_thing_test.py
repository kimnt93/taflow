import numpy as np
from taflow import KnowSureThing


def test_know_sure_thing_lifecycle() -> None:
    close = 100.0 + np.arange(128.0) * 0.2
    state = KnowSureThing(close, roc1=3, roc2=4, roc3=5, roc4=6, sma1=3, sma2=3, sma3=3, sma4=3, signal=3)
    first = state.compute()
    state.reset().extend(close)
    for got, expected in zip(state.compute(), first):
        np.testing.assert_array_equal(got, expected)

