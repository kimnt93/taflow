import numpy as np
from taflow import AverageTrueRangeBands
def test_average_true_range_bands_lifecycle():
    s=AverageTrueRangeBands(np.array([],float),np.array([],float),np.array([],float),2,2);s.extend([2,2],[1,1],[1.5,1.5]);assert s.value is not None;s.reset();assert len(s)==0
