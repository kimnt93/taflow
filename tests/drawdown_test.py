import numpy as np

from taflow import Drawdown


def test_running_drawdown_and_reset() -> None:
    values = np.array([10.0, 8.0, 12.0, 9.0])
    state = Drawdown(values)
    np.testing.assert_allclose(state.compute(), [0.0, -0.2, 0.0, -0.25])
    assert state.reset().value is None
