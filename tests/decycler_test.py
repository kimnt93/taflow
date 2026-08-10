import numpy as np
from taflow import Decycler
def test_decycler_lifecycle():
    s=Decycler(np.array([],float),3);s.extend([1]);assert s.value is not None;s.reset();assert len(s)==0
