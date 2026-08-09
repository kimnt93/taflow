import numpy as np

from taflow import RollingSharpe


def test_matches_reference() -> None:
    values = np.array([1.0, 2.0, 3.0, 2.0, 4.0])
    actual = RollingSharpe(values, 3).compute()
    expected = np.full(values.size, np.nan)
    for index in range(2, values.size):
        window = values[index - 2 : index + 1]
        expected[index] = window.mean() / window.std() if window.std() else 0.0
    np.testing.assert_allclose(actual, expected, equal_nan=True)
