import numpy as np
from taflow import WilliamsAccumulationDistribution
def test_williams_accumulation_distribution_lifecycle():
    s=WilliamsAccumulationDistribution(np.array([],float),np.array([],float),np.array([],float));s.extend([2],[1],[1.5]);assert s.value is not None;s.reset();assert len(s)==0
