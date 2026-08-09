import numpy as np

from taflow import Squeeze


def test_squeeze_lifecycle() -> None:
    high = 50.0 + np.sin(np.arange(96.0) / 5.0)
    low = high - 2.0
    close = high - 1.0
    state = Squeeze(high, low, close)
    first = state.compute()
    assert np.isfinite(first[0]).any()
    state.reset().extend(high, low, close)
    for got, expected in zip(state.compute(), first):
        np.testing.assert_array_equal(got, expected)
    assert len(state) == len(close)

