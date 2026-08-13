import numpy as np
from taflow import RollingVarianceRatio
def test_rolling_variance_ratio_lifecycle():
    state=RollingVarianceRatio(2, 2);state.extend([1,2],[2,1]);assert state.value is not None;state.reset();assert len(state)==0
