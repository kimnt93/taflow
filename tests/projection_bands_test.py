import numpy as np
from taflow import ProjectionBands
def test_projection_bands_lifecycle():
    s=ProjectionBands(2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
