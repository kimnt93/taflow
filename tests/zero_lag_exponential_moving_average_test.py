import numpy as np

from taflow import ZeroLagExponentialMovingAverage


def test_lifecycle() -> None:
    values = np.linspace(1.0, 20.0, 32)
    state = ZeroLagExponentialMovingAverage(values, 9)
    assert len(state) == len(values)
    assert state.reset().value is None
