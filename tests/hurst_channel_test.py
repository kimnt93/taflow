import numpy as np
from taflow import HurstChannel
def test_hurst_channel_lifecycle():
    s=HurstChannel(np.array([],float),np.array([],float),np.array([],float),2,.5);s.extend([2,3],[1,2],[1.5,2.5]);assert s.value is not None;s.reset();assert len(s)==0
