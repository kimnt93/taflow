import numpy as np
from taflow import LinearRegressionChannel
def test_linear_regression_channel_lifecycle():
    s=LinearRegressionChannel(np.array([],float),2);s.extend([1,2]);assert s.value is not None;s.reset();assert len(s)==0
