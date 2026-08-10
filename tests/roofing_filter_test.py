import numpy as np
from taflow import RoofingFilter
def test_roofing_filter_lifecycle():
    s=RoofingFilter(np.array([],float),3,5);s.extend([1]);assert s.value is not None;s.reset();assert len(s)==0
