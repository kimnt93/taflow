import numpy as np

from taflow import Retracements


def test_retracements_lifecycle_and_alignment():
    close = np.linspace(10.0, 20.0, 80)
    high = close + 0.5
    low = close - 0.5
    indicator = Retracements(high, low, close, swing_length=3)
    outputs = indicator.compute()
    assert all(array.shape == close.shape for array in outputs)
    assert len(indicator) == len(close)
    indicator.reset().extend(high[:20], low[:20], close[:20])
    assert len(indicator) == 20


def test_retracements_rejects_misaligned_inputs():
    with np.testing.assert_raises(ValueError):
        Retracements([1, 2], [0], [1, 2])
