import numpy as np

from taflow import RollingAlpha


def test_rolling_alpha_lifecycle_and_alignment():
    state = RollingAlpha(
        np.array([], dtype=float), np.array([], dtype=float), timeperiod=2
    )
    state.extend([1.0, 2.0], [2.0, 4.0])
    np.testing.assert_allclose(state.compute(), [np.nan, 0.0], equal_nan=True)
    assert len(state) == 2
    state.reset()
    assert state.value is None
