import numpy as np
from taflow import SuperSmoother
def test_super_smoother_lifecycle():
    s=SuperSmoother(np.array([],float),3);s.extend([1]);assert s.value is not None;s.reset();assert len(s)==0
