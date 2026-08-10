import numpy as np
from taflow import RollingStandardError
def test_rolling_standard_error_lifecycle():
    s=RollingStandardError(np.array([],float),2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
