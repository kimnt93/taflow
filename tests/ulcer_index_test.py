import numpy as np

from taflow import UlcerIndex


def test_lifecycle() -> None:
    values = np.linspace(10.0, 1.0, 32)
    state = UlcerIndex(values, 5)
    assert len(state) == len(values)
    assert state.reset().value is None
