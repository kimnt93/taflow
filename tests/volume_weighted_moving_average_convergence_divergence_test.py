import numpy as np
from taflow import VolumeWeightedMovingAverageConvergenceDivergence
def test_volume_weighted_moving_average_convergence_divergence_lifecycle():
    s=VolumeWeightedMovingAverageConvergenceDivergence(np.array([],float),np.array([],float),2,3);s.extend([1,2,3],[2,2,2]);assert s.value is not None;s.reset();assert len(s)==0
