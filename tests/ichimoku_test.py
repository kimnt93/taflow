import numpy as np

from taflow import Ichimoku


def test_ichimoku_lifecycle() -> None:
    high = 50.0 + np.arange(80.0)
    low = high - 2.0
    close = high - 1.0
    state = Ichimoku().extend(high, low, close)
    first = state.compute()
    assert np.isnan(first[0][0])
    state.reset().extend(high, low, close)
    for got, expected in zip(state.compute(), first):
        np.testing.assert_array_equal(got, expected)
    assert len(state) == len(close)

