import numpy as np
from taflow import RollingBetaNeutralSpread
def test_rolling_beta_neutral_spread_lifecycle():
    s=RollingBetaNeutralSpread(np.array([],float),np.array([],float),2);s.extend([1,2],[2,4]);assert s.value is not None;s.reset();assert len(s)==0
