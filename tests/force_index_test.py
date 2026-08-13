import numpy as np

from taflow import ForceIndex


def test_force_index_lifecycle_and_alignment():
    state = ForceIndex()
    close = np.arange(14.0) + 10.0
    volume = np.arange(14.0) + 2.0
    state.extend(close, volume)

    assert np.isnan(state.compute()[:13]).all()
    assert np.isfinite(state.compute()[13])
    assert len(state) == 14
    state.reset()
    assert state.value is None
