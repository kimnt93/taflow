import numpy as np
from taflow import AdaptiveCycle
def test_adaptive_cycle_lifecycle():
    s=AdaptiveCycle(np.array([],float));s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
