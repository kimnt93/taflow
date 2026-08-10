import numpy as np
from taflow import RollingGainLossRatio
def test_rolling_gain_loss_ratio_lifecycle():
    state=RollingGainLossRatio(np.array([],dtype=float),2);state.extend([1,-1]);assert state.value is not None;state.reset();assert len(state)==0
