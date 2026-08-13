import numpy as np

from taflow import FractalDimension


def test_fractal_dimension_lifecycle_and_reset():
    state = FractalDimension(timeperiod=4)
    state.extend([1.0, 2.0, 3.0, 4.0])
    assert np.isfinite(state.compute()[-1])
    assert len(state) == 4
    state.reset()
    assert state.value is None
