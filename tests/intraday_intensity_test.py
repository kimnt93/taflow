import numpy as np
from taflow import IntradayIntensity
def test_intraday_intensity_lifecycle():
    s=IntradayIntensity();s.extend([2],[1],[1.5],[10]);assert s.value is not None;s.reset();assert len(s)==0
