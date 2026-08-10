import numpy as np
from taflow import RollingAverageDrawdown
def test_rolling_average_drawdown_lifecycle():
    state=RollingAverageDrawdown(np.array([],dtype=float),3);state.extend([3,2,1]);assert state.value is not None;state.reset();assert len(state)==0
