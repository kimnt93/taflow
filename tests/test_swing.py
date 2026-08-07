import numpy as np
import pytest

from taflow import SwingHighLow


def test_swing_confirmation_is_causal_and_chunk_invariant():
    high = np.array([1.0, 3.0, 5.0, 4.0, 3.0, 2.0, 4.0, 1.0])
    low = np.array([0.0, 1.0, 2.0, 1.0, 0.0, -1.0, 1.0, 0.0])
    expected_signal = np.array([np.nan, np.nan, np.nan, np.nan, 1.0, np.nan, np.nan, -1.0])
    expected_level = np.array([np.nan, np.nan, np.nan, np.nan, 5.0, np.nan, np.nan, -1.0])
    expected_bars = np.array([np.nan, np.nan, np.nan, np.nan, 0.0, 1.0, 2.0, 0.0])

    state = SwingHighLow(swing_length=2)
    actual = state.extend(high, low).compute()
    for result, expected in zip(actual, (expected_signal, expected_level, expected_bars)):
        np.testing.assert_array_equal(result, expected)

    chunked = SwingHighLow(swing_length=2)
    chunked.extend(high[:4], low[:4]).extend(high[4:], low[4:])
    for result, expected in zip(chunked.compute(), actual):
        np.testing.assert_array_equal(result, expected)


def test_swing_reset_and_input_validation():
    with pytest.raises(ValueError):
        SwingHighLow(swing_length=0)
    with pytest.raises(ValueError):
        SwingHighLow().extend(np.ones(3), np.ones(2))

    state = SwingHighLow(swing_length=1)
    state.extend(np.array([1.0, 3.0, 1.0]), np.array([0.0, 1.0, 0.0]))
    state.reset()
    assert state.value is None
    assert all(len(values) == 0 for values in state.compute())
