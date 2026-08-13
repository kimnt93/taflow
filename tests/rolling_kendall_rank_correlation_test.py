import numpy as np
from taflow import RollingKendallRankCorrelation
def test_rolling_kendall_rank_correlation_lifecycle():
    state=RollingKendallRankCorrelation(2);state.extend([1,2],[2,3]);assert state.value is not None;state.reset();assert len(state)==0
