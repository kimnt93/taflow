import numpy as np

from taflow import TrueStrengthIndex


def test_lifecycle() -> None:
    values = np.linspace(1.0, 20.0, 32)
    state = TrueStrengthIndex(values, 5, 10)
    assert len(state) == len(values)
    assert state.reset().value is None
