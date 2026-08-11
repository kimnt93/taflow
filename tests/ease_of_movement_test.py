import numpy as np

from taflow import EaseOfMovement


def test_ease_of_movement_lifecycle_and_alignment():
    state = EaseOfMovement(
        np.array([], dtype=float), np.array([], dtype=float), np.array([], dtype=float)
    )
    high = np.arange(15.0) + 11.0
    low = high - 2.0
    volume = np.full(15, 2.0)
    state.extend(high, low, volume)

    assert np.isnan(state.compute()[:14]).all()
    assert np.isfinite(state.compute()[14])
    assert len(state) == 15
    state.reset()
    assert state.value is None
