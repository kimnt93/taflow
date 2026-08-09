import numpy as np

from taflow import PreviousHighLow


def test_previous_high_low_lifecycle_and_reset():
    state = PreviousHighLow(np.array([], dtype=bool), np.array([], dtype=float), np.array([], dtype=float))
    state.extend([True, True], [10.0, 12.0], [8.0, 7.0])
    assert state.compute()[0][-1] == 10.0
    assert len(state) == 2
    state.reset()
    assert state.value is None
