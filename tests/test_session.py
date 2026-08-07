import numpy as np
import pytest

from taflow import SessionExtrema, session_flags


def test_session_extrema_resets_at_explicit_boundaries():
    session_id = np.array([1, 1, 1, 2, 2])
    flags = session_flags(session_id)
    state = SessionExtrema()
    high = np.array([3.0, 2.0, 4.0, 1.0, 5.0])
    low = np.array([1.0, 0.0, 2.0, -1.0, 3.0])
    actual = state.extend(flags, high, low).compute()
    np.testing.assert_array_equal(actual[0], [3.0, 3.0, 4.0, 1.0, 5.0])
    np.testing.assert_array_equal(actual[1], [1.0, 0.0, 0.0, -1.0, -1.0])


def test_session_extrema_rejects_mismatched_inputs():
    with pytest.raises(ValueError):
        SessionExtrema().extend([True, False], [1.0], [0.0, -1.0])
