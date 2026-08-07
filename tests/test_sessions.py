import numpy as np
import pytest

from taflow import Sessions, session_flags


def test_sessions_tracks_running_extrema_per_session():
    session_id = np.array([1, 1, 1, 2, 2, 2, 3, 3])
    high = np.array([10.0, 11.0, 9.0, 12.0, 11.0, 13.0, 8.0, 9.0])
    low = np.array([9.0, 10.0, 8.0, 11.0, 10.0, 12.0, 7.0, 8.0])
    active, session_high, session_low = Sessions().extend(session_flags(session_id), high, low).compute()
    np.testing.assert_array_equal(active, np.ones(8))
    np.testing.assert_array_equal(session_high, [10.0, 11.0, 11.0, 12.0, 12.0, 13.0, 8.0, 9.0])
    np.testing.assert_array_equal(session_low, [9.0, 9.0, 8.0, 11.0, 10.0, 10.0, 7.0, 7.0])


def test_sessions_rejects_mismatched_inputs_and_resets():
    state = Sessions()
    with pytest.raises(ValueError):
        state.extend(np.array([True, False]), np.ones(2), np.ones(1))
    state.extend(np.array([True, False]), np.ones(2), np.zeros(2))
    state.reset()
    assert all(len(values) == 0 for values in state.compute())
