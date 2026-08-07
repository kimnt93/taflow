import numpy as np

from taflow import Dpo


def test_dpo_matches_causal_reference_and_chunks():
    close = 100.0 + np.cumsum(np.sin(np.arange(200.0)) * 0.2)
    period = 20
    delay = period // 2 + 1
    expected = np.full(len(close), np.nan)
    for index in range(period - 1 + delay, len(close)):
        expected[index] = close[index] - close[index - delay - period + 1 : index - delay + 1].mean()

    full = Dpo(close=close, period=period).compute()
    np.testing.assert_allclose(full, expected, equal_nan=True, atol=1e-12)

    chunked = Dpo(period=period)
    for start in range(0, len(close), 17):
        chunked.extend(close[start : start + 17])
    np.testing.assert_array_equal(chunked.compute(), full)


def test_dpo_reset():
    close = np.arange(100.0)
    state = Dpo(close=close)
    expected = state.compute()
    state.reset().extend(close)
    np.testing.assert_array_equal(state.compute(), expected)
