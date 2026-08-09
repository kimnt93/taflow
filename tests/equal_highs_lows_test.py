import numpy as np

from taflow import EqualHighsLows


def test_equal_highs_lows_lifecycle_and_alignment():
    close = np.linspace(10.0, 20.0, 80)
    high = close + 0.5
    low = close - 0.5
    indicator = EqualHighsLows(high, low, close, eq_len=3, atr_period=14)
    outputs = indicator.compute()
    assert all(array.shape == close.shape for array in outputs)
    assert len(indicator) == len(close)
    indicator.reset()
    indicator.extend(high[:20], low[:20], close[:20])
    assert len(indicator) == 20


def test_equal_highs_lows_rejects_misaligned_inputs():
    with np.testing.assert_raises(ValueError):
        EqualHighsLows([1, 2], [0], [1, 2])
