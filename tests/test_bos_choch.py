import numpy as np
import pytest

from taflow import BosChoch


def test_bos_choch_is_aligned_and_chunk_invariant():
    high = np.array([1.0, 5.0, 2.0, 6.0, 3.0, 7.0, 8.0])
    low = np.array([0.0, 4.0, 1.0, 3.0, 2.0, 5.0, 6.0])
    close = np.array([0.5, 4.5, 1.5, 5.5, 2.5, 6.5, 7.5])
    state = BosChoch(swing_length=1)
    actual = state.extend(high, low, close).compute()
    assert all(len(values) == len(high) for values in actual)
    chunked = BosChoch(swing_length=1)
    chunked.extend(high[:3], low[:3], close[:3]).extend(high[3:], low[3:], close[3:])
    for ours, theirs in zip(chunked.compute(), actual):
        np.testing.assert_array_equal(ours, theirs)


def test_bos_choch_rejects_invalid_inputs():
    with pytest.raises(ValueError):
        BosChoch(swing_length=0)
