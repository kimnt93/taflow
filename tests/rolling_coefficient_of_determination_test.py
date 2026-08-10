import numpy as np
from taflow import RollingCoefficientOfDetermination
def test_rolling_coefficient_of_determination_lifecycle():
    s=RollingCoefficientOfDetermination(np.array([],float),np.array([],float),2);s.extend([1,2],[2,4]);assert s.value is not None;s.reset();assert len(s)==0
