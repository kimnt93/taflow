import numpy as np

from taflow import HilbertTransformTrendMode


def test_hilbert_transform_trend_mode_lifecycle():
    values = np.sin(np.linspace(0.0, 10.0, 100))
    indicator = HilbertTransformTrendMode(values)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None
