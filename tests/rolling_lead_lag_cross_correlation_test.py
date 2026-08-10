import numpy as np
from taflow import RollingLeadLagCrossCorrelation
def test_rolling_lead_lag_cross_correlation_lifecycle():
    s=RollingLeadLagCrossCorrelation(np.array([],float),np.array([],float),2,1);s.extend([1,2,3],[2,3,4]);assert s.value is not None;s.reset();assert len(s)==0
