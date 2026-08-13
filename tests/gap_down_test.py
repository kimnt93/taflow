import numpy as np

from taflow import GapDown


def test_detects_gap_downs() -> None:
    high = np.array([10.0, 7.0, 9.0])
    low = np.array([8.0, 6.0, 7.0])
    np.testing.assert_allclose(GapDown().extend(high, low).compute(), [np.nan, 1.0, 0.0], equal_nan=True)
