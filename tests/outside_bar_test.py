import numpy as np

from taflow import OutsideBar


def test_detects_outside_bars() -> None:
    high = np.array([10.0, 11.0, 10.0])
    low = np.array([8.0, 7.0, 8.0])
    np.testing.assert_allclose(OutsideBar(high, low).compute(), [np.nan, 1.0, 0.0], equal_nan=True)
