import numpy as np

from taflow.ease_of_movement import EaseOfMovement


def test_ease_of_movement_lifecycle_and_alignment():
    state = EaseOfMovement(
        np.array([], dtype=float), np.array([], dtype=float), np.array([], dtype=float)
    )
    state.extend([11.0, 12.0], [9.0, 10.0], [2.0, 2.0])
    np.testing.assert_allclose(state.compute(), [np.nan, 1.0], equal_nan=True)
    assert len(state) == 2
    state.reset()
    assert state.value is None
