import numpy as np
from taflow import RollingSpearmanCorrelation
def test_rolling_spearman_correlation_lifecycle():
    state=RollingSpearmanCorrelation(np.array([],dtype=float),np.array([],dtype=float),2);state.extend([1,2],[2,3]);assert state.value is not None;state.reset();assert len(state)==0
