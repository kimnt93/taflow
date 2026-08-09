import numpy as np

from taflow.swing import SwingHighLow


def test_swing_high_low_lifecycle_and_reset():
    state = SwingHighLow(np.array([], dtype=float), np.array([], dtype=float), swing_length=2)
    state.extend([10, 11, 12, 11, 10], [8, 7, 6, 7, 8])
    assert len(state.compute()) == 3
    state.reset()
    assert state.value is None

