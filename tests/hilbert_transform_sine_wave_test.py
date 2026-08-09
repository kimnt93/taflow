import numpy as np

from taflow import HilbertTransformSineWave


def test_hilbert_transform_sine_wave_lifecycle():
    values = np.sin(np.linspace(0.0, 10.0, 100))
    indicator = HilbertTransformSineWave(values)
    sine, leadsine = indicator.compute()
    assert len(sine) == len(values)
    assert len(leadsine) == len(values)
    indicator.reset()
    assert indicator.value is None
