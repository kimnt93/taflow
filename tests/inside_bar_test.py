import numpy as np

from taflow import InsideBar


def test_detects_inside_bars() -> None:
    high = np.array([10.0, 9.0, 11.0, 10.0])
    low = np.array([8.0, 8.5, 7.0, 7.5])
    np.testing.assert_allclose(InsideBar(high, low).compute(), [np.nan, 1.0, 0.0, 1.0], equal_nan=True)
