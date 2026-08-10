import numpy as np
from taflow import CenterOfGravity
def test_center_of_gravity_lifecycle():
    s=CenterOfGravity(np.array([],float),2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
