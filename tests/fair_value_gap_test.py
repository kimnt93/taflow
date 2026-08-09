import numpy as np

from taflow.fair_value_gap import FairValueGap


def test_fair_value_gap_lifecycle_and_reset():
    state = FairValueGap(np.array([], dtype=float), np.array([], dtype=float), np.array([], dtype=float), np.array([], dtype=float))
    state.extend([10.0, 10.5], [11.0, 12.0], [9.0, 10.0], [10.5, 11.5])
    assert len(state) == 2
    assert len(state.compute()) == 4
    state.reset()
    assert state.value is None

