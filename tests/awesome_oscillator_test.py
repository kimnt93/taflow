import numpy as np

from taflow import AwesomeOscillator


def test_lifecycle() -> None:
    high = np.arange(32, dtype=float) + 10.0
    low = high - 1.0
    state = AwesomeOscillator(3, 5).extend(high, low)
    assert len(state) == len(high)
    assert state.reset().value is None
