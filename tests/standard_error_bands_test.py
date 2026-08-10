import numpy as np
from taflow import StandardErrorBands
def test_standard_error_bands_lifecycle():
    s=StandardErrorBands(np.array([],float),2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
