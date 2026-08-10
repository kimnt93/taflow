import numpy as np

from taflow import RollingSharpe
from tests.oracle_assertions import assert_registered_oracle_match


def test_matches_reference() -> None:
    values = np.array([1.0, 2.0, 3.0, 2.0, 4.0])
    actual = RollingSharpe(values, 3).compute()
    expected = np.full(values.size, np.nan)
    for index in range(2, values.size):
        window = values[index - 2 : index + 1]
        deviation = window.std(ddof=1)
        expected[index] = window.mean() / deviation if deviation else 0.0
    np.testing.assert_allclose(actual, expected, equal_nan=True)


def test_rolling_sharpe_matches_wickra() -> None:
    assert_registered_oracle_match("RollingSharpe")
