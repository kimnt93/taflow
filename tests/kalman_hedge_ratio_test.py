import numpy as np

from taflow import KalmanHedgeRatio


def test_kalman_hedge_ratio_lifecycle() -> None:
    x = np.arange(64.0) / 8.0
    y = 1.0 + 2.0 * x
    state = KalmanHedgeRatio(x, y)
    first = state.compute()
    assert first[-1] > 1.5
    state.reset().extend(x, y)
    np.testing.assert_array_equal(state.compute(), first)
    assert len(state) == len(x)

