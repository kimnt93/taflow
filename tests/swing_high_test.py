import numpy as np

from taflow.swing_high import SwingHigh


def test_swing_high_lifecycle_and_reset():
    state = SwingHigh(np.array([], dtype=float), np.array([], dtype=float), swing_length=2)
    state.extend([10, 11, 12, 11, 10], [8, 7, 6, 7, 8])
    assert len(state) == 5
    state.reset()
    assert state.value is None

