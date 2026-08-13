import numpy as np

from taflow import HigherHigh


def test_detects_higher_highs() -> None:
    high = np.array([10.0, 11.0, 10.0, 12.0])
    low = np.array([8.0, 9.0, 8.0, 10.0])
    np.testing.assert_allclose(HigherHigh().extend(high, low).compute(), [np.nan, 1.0, 0.0, 1.0], equal_nan=True)
