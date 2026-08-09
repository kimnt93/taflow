import numpy as np

from taflow import FisherTransform


def test_lifecycle() -> None:
    high = np.arange(32, dtype=float) + 10.0
    low = high - 1.0
    state = FisherTransform(high, low, 5)
    assert len(state) == len(high)
    assert state.reset().value is None
