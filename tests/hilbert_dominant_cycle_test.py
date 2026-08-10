import numpy as np
from taflow import HilbertDominantCycle
def test_lifecycle():
    x=HilbertDominantCycle(np.arange(40.0));assert len(x)==40;x.reset();assert len(x)==0 and x.value is None
