import numpy as np

from taflow.force_index import ForceIndex


def test_force_index_lifecycle_and_alignment():
    state = ForceIndex(np.array([], dtype=float), np.array([], dtype=float))
    state.extend([10.0, 11.0], [2.0, 3.0])
    np.testing.assert_allclose(state.compute(), [np.nan, 3.0], equal_nan=True)
    assert len(state) == 2
    state.reset()
    assert state.value is None
