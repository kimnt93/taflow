import numpy as np

from taflow import VolumeWeightedMovingAverageConvergenceDivergence


def test_volume_weighted_moving_average_convergence_divergence_lifecycle():
    state = VolumeWeightedMovingAverageConvergenceDivergence(fast=2, slow=3, signal=1)
    state.extend([1.0, 2.0, 3.0], [2.0, 2.0, 2.0])

    assert state.value is not None
    state.reset()
    assert len(state) == 0
