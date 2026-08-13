import numpy as np

from taflow import RollingSortino


def test_matches_reference() -> None:
    values = np.array([1.0, -2.0, 3.0, -1.0, 4.0])
    actual = RollingSortino(3).extend(values).compute()
    expected = np.full(values.size, np.nan)
    for index in range(2, values.size):
        window = values[index - 2 : index + 1]
        downside = np.minimum(window, 0.0)
        denominator = np.sqrt(np.mean(downside**2))
        expected[index] = window.mean() / denominator if denominator else 0.0
    np.testing.assert_allclose(actual, expected, equal_nan=True)
