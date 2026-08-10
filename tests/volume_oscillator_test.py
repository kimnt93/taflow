import numpy as np
from taflow import VolumeOscillator
def test_lifecycle():
    state=VolumeOscillator(np.array([],dtype=float),2,3); state.extend([1.0,2.0,3.0]); assert state.value is not None; state.reset(); assert len(state)==0
