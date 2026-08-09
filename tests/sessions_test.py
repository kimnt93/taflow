import numpy as np

from taflow.sessions import Sessions


def test_sessions_lifecycle_and_reset():
    state = Sessions(np.array([], dtype=bool), np.array([], dtype=float), np.array([], dtype=float))
    state.extend([True, False], [10.0, 12.0], [8.0, 7.0])
    assert state.compute()[1][-1] == 12.0
    assert len(state) == 2
    state.reset()
    assert state.value is None

