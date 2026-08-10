import numpy as np
from taflow import InstantaneousTrendline
def test_instantaneous_trendline_lifecycle():
    s=InstantaneousTrendline(np.array([],float),3);s.extend([1]);assert s.value is not None;s.reset();assert len(s)==0
