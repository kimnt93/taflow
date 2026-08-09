import numpy as np

from taflow import CloseToCloseSigma


def test_close_to_close_sigma_lifecycle():
    close = np.linspace(100.0, 120.0, 80)
    indicator = CloseToCloseSigma(close, timeperiod=10)
    output = indicator.compute()
    assert output.shape == close.shape
    assert len(indicator) == len(close)
    indicator.reset().extend(close[:20])
    assert len(indicator) == 20
