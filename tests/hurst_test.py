import numpy as np

from taflow import Hurst


def test_hurst_lifecycle_and_reset():
    state = Hurst(np.array([], dtype=float), timeperiod=3)
    state.extend([1.0, 2.0, 3.0])
    assert np.isfinite(state.compute()[-1])
    assert len(state) == 3
    state.reset()
    assert state.value is None
