import numpy as np
from taflow import RollingPainIndex
def test_rolling_pain_index_lifecycle():
    state=RollingPainIndex(np.array([],dtype=float),2);state.extend([2,1]);assert state.value is not None;state.reset();assert len(state)==0
