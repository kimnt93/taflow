import numpy as np

from taflow import Supertrend


def test_supertrend_lifecycle() -> None:
    high = 100.0 + np.sin(np.arange(96.0) / 10.0)
    low = high - 2.0
    close = high - 1.0
    state = Supertrend().extend(high, low, close)
    first = state.compute()
    assert np.isnan(first[0][:6]).all()
    state.reset().extend(high, low, close)
    for got, expected in zip(state.compute(), first):
        np.testing.assert_array_equal(got, expected)
    assert len(state) == len(close)

