import numpy as np

from taflow import LowerLow


def test_detects_lower_lows() -> None:
    high = np.array([10.0, 11.0, 10.0, 12.0])
    low = np.array([8.0, 7.0, 8.0, 6.0])
    np.testing.assert_allclose(LowerLow().extend(high, low).compute(), [np.nan, 1.0, 0.0, 1.0], equal_nan=True)
