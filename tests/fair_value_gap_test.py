import numpy as np

from taflow import FairValueGap


def test_fair_value_gap_lifecycle_and_reset():
    state = FairValueGap()
    state.extend([10.0, 10.5], [11.0, 12.0], [9.0, 10.0], [10.5, 11.5])
    assert len(state) == 2
    assert len(state.compute()) == 4
    state.reset()
    assert state.value is None
