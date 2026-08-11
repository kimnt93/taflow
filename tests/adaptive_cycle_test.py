import numpy as np

from taflow import AdaptiveCycle


def test_adaptive_cycle_lifecycle():
    state = AdaptiveCycle(np.array([], dtype=float))
    state.extend(np.arange(60.0))

    assert state.value is not None
    state.reset()
    assert len(state) == 0
