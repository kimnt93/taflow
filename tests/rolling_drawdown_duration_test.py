import numpy as np
from taflow import RollingDrawdownDuration
def test_rolling_drawdown_duration_lifecycle():
    state=RollingDrawdownDuration(np.array([],dtype=float));state.extend([2,1]);assert state.value==1;state.reset();assert len(state)==0
