import numpy as np
import polars as pl

from taflow import CumulativeMinimum


def test_cumulative_minimum_matches_polars_and_lifecycle() -> None:
    values = np.array([2.0, 4.0, 1.0, 8.0, 2.0, -3.0, 5.0], dtype=np.float64)
    expected = pl.Series(values).cum_min().to_numpy()
    np.testing.assert_array_equal(CumulativeMinimum().extend(values).compute(), expected)

    state = CumulativeMinimum()
    assert state.value is None
    assert state.extend(values[:3]) is state
    assert state.extend(values[3:]) is state
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for value in values:
        assert state.append(float(value)) is state
    np.testing.assert_array_equal(state.compute(), expected)
