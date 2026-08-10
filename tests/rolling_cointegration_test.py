import numpy as np
from taflow import RollingCointegration
def test_rolling_cointegration_lifecycle():
    state=RollingCointegration(np.array([],dtype=float),np.array([],dtype=float),2);state.extend([1,2],[2,4]);assert state.value is not None;state.reset();assert len(state)==0
