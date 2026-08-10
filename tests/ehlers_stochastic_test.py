import numpy as np
from taflow import EhlersStochastic
def test_ehlers_stochastic_lifecycle():
    s=EhlersStochastic(np.array([],float),2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
