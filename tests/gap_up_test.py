import numpy as np

from taflow import GapUp


def test_detects_gap_ups() -> None:
    high = np.array([10.0, 12.0, 13.0])
    low = np.array([8.0, 11.0, 12.0])
    np.testing.assert_allclose(GapUp().extend(high, low).compute(), [np.nan, 1.0, 0.0], equal_nan=True)
