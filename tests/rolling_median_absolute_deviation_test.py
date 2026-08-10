import numpy as np
from taflow import RollingMedianAbsoluteDeviation
def test_rolling_median_absolute_deviation_lifecycle():
    s=RollingMedianAbsoluteDeviation(np.array([],float),2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
