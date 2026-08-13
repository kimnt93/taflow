import numpy as np
import talib

from taflow import MovingAverageConvergenceDivergence


def test_matches_talib_and_lifecycle() -> None:
    close = 100.0 + np.sin(np.arange(300) * 0.13).cumsum()
    oracle = talib.MACD(close, 12, 26, 9)
    expected = MovingAverageConvergenceDivergence().extend(close).compute()
    for got, want in zip(expected, oracle):
        np.testing.assert_allclose(got, want, rtol=0.0, atol=2e-12, equal_nan=True)

    state = MovingAverageConvergenceDivergence(12, 26, 9)
    for chunk in np.array_split(close, 5):
        assert state.extend(chunk) is state
    for got, want in zip(state.compute(), expected):
        np.testing.assert_array_equal(got, want)
    final = state.value
    assert state.reset() is state
    state.extend(close)
    assert state.value == final
