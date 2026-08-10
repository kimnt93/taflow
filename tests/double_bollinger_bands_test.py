import numpy as np
from taflow import DoubleBollingerBands
def test_double_bollinger_bands_lifecycle():
    s=DoubleBollingerBands(np.array([],float),2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
