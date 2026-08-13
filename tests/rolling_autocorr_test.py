import numpy as np

from taflow import RollingAutocorr


def test_rolling_autocorr_lifecycle_and_reset():
    state = RollingAutocorr(timeperiod=3)
    state.extend([1.0, 2.0, 3.0])
    assert np.isfinite(state.compute()[-1])
    assert len(state) == 3
    state.reset()
    assert state.value is None
