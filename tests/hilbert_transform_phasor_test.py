import numpy as np

from taflow import HilbertTransformPhasor


def test_hilbert_transform_phasor_lifecycle():
    values = np.sin(np.linspace(0.0, 10.0, 100))
    indicator = HilbertTransformPhasor(values)
    inphase, quadrature = indicator.compute()
    assert len(inphase) == len(values)
    assert len(quadrature) == len(values)
    indicator.reset()
    assert indicator.value is None
