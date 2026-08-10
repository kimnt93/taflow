import numpy as np
from taflow import VolumeZoneOscillator
def test_lifecycle():
    state=VolumeZoneOscillator(np.array([],dtype=float),np.array([],dtype=float),3); state.extend([1.0,2.0,1.5],[10.0,11.0,12.0]); assert state.value is not None; state.reset(); assert len(state)==0
