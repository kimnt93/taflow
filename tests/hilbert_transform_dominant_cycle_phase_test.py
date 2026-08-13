import numpy as np

from taflow import HilbertTransformDominantCyclePhase


def test_hilbert_transform_dominant_cycle_phase_lifecycle():
    values = np.sin(np.linspace(0.0, 10.0, 100))
    indicator = HilbertTransformDominantCyclePhase().extend(values)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None
