import numpy as np

from taflow import RollingLeadLagCrossCorrelation


def test_rolling_lead_lag_cross_correlation_lifecycle():
    state = RollingLeadLagCrossCorrelation(window=2, max_lag=1)
    state.extend([1.0, 2.0, 3.0, 4.0], [2.0, 3.0, 4.0, 5.0])

    assert state.value is not None
    state.reset()
    assert len(state) == 0
