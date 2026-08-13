import numpy as np

from taflow import HilbertTransformDominantCyclePeriod


def test_hilbert_transform_dominant_cycle_period_lifecycle():
    values = np.linspace(100.0, 110.0, 100)
    indicator = HilbertTransformDominantCyclePeriod().extend(values)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None
