import numpy as np

from taflow import ArnaudLegouxMovingAverage


def test_lifecycle() -> None:
    values = np.linspace(1.0, 20.0, 32)
    state = ArnaudLegouxMovingAverage(9, 0.85, 6.0).extend(values)
    assert len(state) == len(values)
    assert state.reset().value is None
