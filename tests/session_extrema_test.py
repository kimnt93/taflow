import numpy as np

from taflow import SessionExtrema


def test_session_extrema_lifecycle_and_reset():
    state = SessionExtrema(np.array([], dtype=bool), np.array([], dtype=float), np.array([], dtype=float))
    state.extend([True, False], [10.0, 12.0], [8.0, 7.0])
    high, low = state.compute()
    np.testing.assert_allclose(high, [10.0, 12.0])
    np.testing.assert_allclose(low, [8.0, 7.0])
    assert len(state) == 2
    state.reset()
    assert state.value is None
