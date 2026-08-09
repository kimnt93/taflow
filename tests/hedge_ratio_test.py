import numpy as np

from taflow.hedge_ratio import HedgeRatio


def test_hedge_ratio_lifecycle_and_alignment():
    state = HedgeRatio(np.array([], dtype=float), np.array([], dtype=float), timeperiod=2)
    state.extend([1.0, 2.0], [2.0, 4.0])
    np.testing.assert_allclose(state.compute(), [np.nan, 2.0], equal_nan=True)
    assert len(state) == 2
    state.reset()
    assert state.value is None

