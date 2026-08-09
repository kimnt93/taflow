import numpy as np

from taflow import DetrendedPriceOscillator


def test_detrended_price_oscillator_lifecycle():
    close = np.linspace(100.0, 120.0, 80)
    indicator = DetrendedPriceOscillator(close, period=10)
    assert indicator.compute().shape == close.shape
    assert len(indicator) == len(close)
    indicator.reset().extend(close[:20])
    assert len(indicator) == 20
