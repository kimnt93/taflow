import numpy as np

from taflow import ExponentiallyWeightedSum


def test_matches_pandas_ewm_sum() -> None:
    values = np.arange(32, dtype=float)
    actual = ExponentiallyWeightedSum(values, 8).compute()
    alpha = 2.0 / 9.0
    expected = np.empty_like(values)
    expected[0] = values[0]
    for index in range(1, len(values)):
        expected[index] = values[index] + (1.0 - alpha) * expected[index - 1]
    np.testing.assert_allclose(actual, expected, equal_nan=True, rtol=1e-12, atol=1e-12)
