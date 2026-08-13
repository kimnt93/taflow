import numpy as np

from taflow import DirectionalMovementIndex


def test_directional_movement_index_lifecycle():
    close = np.linspace(100.0, 110.0, 60)
    indicator = DirectionalMovementIndex().extend(close + 1.0, close - 1.0, close)
    assert len(indicator) == len(close)
    indicator.reset()
    assert indicator.value is None
