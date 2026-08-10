import numpy as np
from taflow import BetterVolume
def test_better_volume_lifecycle():
    s=BetterVolume(np.array([],float),np.array([],float),np.array([],float),np.array([],float));s.extend([2],[1],[1.5],[10]);assert s.value is not None;s.reset();assert len(s)==0
