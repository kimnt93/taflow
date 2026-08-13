import numpy as np

from taflow import HighestSince


def test_tracks_highest_since_true_bar() -> None:
    condition = np.array([False, True, False])
    values = np.array([1.0, 2.0, 1.0])
    np.testing.assert_allclose(HighestSince().extend(condition, values).compute(), [1.0, 2.0, 2.0])
