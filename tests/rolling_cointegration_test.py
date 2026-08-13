import numpy as np

from taflow import RollingCointegration


def test_rolling_cointegration_lifecycle():
    state = RollingCointegration(period=6)
    state.extend(np.arange(6.0), 2.0 * np.arange(6.0) + 1.0)

    assert state.value is not None
    state.reset()
    assert len(state) == 0
