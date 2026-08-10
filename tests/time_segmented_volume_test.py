import numpy as np
from taflow import TimeSegmentedVolume
def test_time_segmented_volume_lifecycle():
    s=TimeSegmentedVolume(np.array([],float),np.array([],float));s.extend([1,2],[10,10]);assert s.value is not None;s.reset();assert len(s)==0
