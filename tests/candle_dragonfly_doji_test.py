import numpy as np

from taflow import CandleDragonflyDoji


def test_dragonfly_doji_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleDragonflyDoji().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None

