import numpy as np

from taflow import RollingMedian


def test_matches_numpy_trailing_median_and_reset() -> None:
    values = np.arange(32, dtype=np.float64) % 7
    actual = RollingMedian(values, 5).compute()
    expected = np.full(values.size, np.nan)
    for index in range(4, values.size):
        expected[index] = np.median(values[index - 4 : index + 1])
    np.testing.assert_allclose(actual, expected, equal_nan=True)
