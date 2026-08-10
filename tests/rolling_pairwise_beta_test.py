import numpy as np
from taflow import RollingPairwiseBeta
def test_rolling_pairwise_beta_lifecycle():
    s=RollingPairwiseBeta(np.array([],float),np.array([],float),2);s.extend([1,2],[2,4]);assert s.value is not None;s.reset();assert len(s)==0
