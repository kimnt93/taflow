import numpy as np

from taflow import HullMovingAverage


def test_lifecycle() -> None:
    values = np.linspace(1.0, 20.0, 32)
    state = HullMovingAverage(values, 9)
    assert len(state) == len(values)
    assert state.reset().value is None
