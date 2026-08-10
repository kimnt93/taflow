import numpy as np
from taflow import DecyclerOscillator
def test_decycler_oscillator_lifecycle():
    s=DecyclerOscillator(np.array([],float),2,4);s.extend([1]);assert s.value is not None;s.reset();assert len(s)==0
