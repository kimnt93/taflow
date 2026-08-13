import numpy as np

from taflow import MesaAdaptiveMovingAverage


def test_mesa_adaptive_moving_average_lifecycle():
    values = np.linspace(100.0, 120.0, 100)
    indicator = MesaAdaptiveMovingAverage().extend(values)
    mama, fama = indicator.compute()
    assert mama.shape == values.shape
    assert fama.shape == values.shape
    assert len(indicator) == len(values)
    indicator.reset().extend(values[:40])
    assert len(indicator) == 40
