import numpy as np
import pytest

from taflow import PreviousHighLow


def test_previous_high_low_tracks_htf_extrema_and_breaks():
    new_session = np.array([True, False, False, True, False, False, True])
    high = np.array([10.0, 12.0, 11.0, 9.0, 9.5, 9.2, 13.0])
    low = np.array([9.0, 8.0, 9.0, 7.0, 8.0, 7.2, 11.0])
    prev_high, prev_low, broken_high, broken_low = PreviousHighLow().extend(new_session, high, low).compute()
    np.testing.assert_array_equal(prev_high, [np.nan, np.nan, np.nan, 12.0, 12.0, 12.0, 9.5])
    np.testing.assert_array_equal(prev_low, [np.nan, np.nan, np.nan, 8.0, 8.0, 8.0, 7.0])
    np.testing.assert_array_equal(broken_high, [np.nan, np.nan, np.nan, np.nan, np.nan, np.nan, 1.0])
    np.testing.assert_array_equal(broken_low, [np.nan, np.nan, np.nan, 1.0, np.nan, 1.0, np.nan])


def test_previous_high_low_rejects_mismatched_inputs_and_resets():
    state = PreviousHighLow()
    with pytest.raises(ValueError):
        state.extend(np.array([True, False]), np.ones(2), np.ones(1))
    state.extend(np.array([True, False]), np.ones(2), np.zeros(2))
    state.reset()
    assert all(len(values) == 0 for values in state.compute())
