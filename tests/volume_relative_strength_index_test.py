import numpy as np
from taflow import VolumeRelativeStrengthIndex
def test_volume_relative_strength_index_lifecycle():
    state=VolumeRelativeStrengthIndex(np.array([],dtype=float),np.array([],dtype=float),3);state.extend([1,2,3,4],[10,20,30,40]);assert state.value is not None;state.reset();assert len(state)==0
