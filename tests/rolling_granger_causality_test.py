import numpy as np
from taflow import RollingGrangerCausality
def test_rolling_granger_causality_lifecycle():
    state=RollingGrangerCausality(np.array([],dtype=float),np.array([],dtype=float),3,1);state.extend([1,2,3],[2,3,4]);assert state.value is not None;state.reset();assert len(state)==0
