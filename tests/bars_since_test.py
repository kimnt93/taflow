import numpy as np

from taflow import BarsSince


def test_matches_causal_condition_count() -> None:
    condition = np.array([False, False, True, False])
    np.testing.assert_allclose(BarsSince(condition).compute(), [0.0, 1.0, 0.0, 1.0])
