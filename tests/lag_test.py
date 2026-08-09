import numpy as np
import pandas as pd

from taflow import Lag


def test_lag_matches_pandas_and_lifecycle() -> None:
    values = np.array([2.0, 4.0, 1.0, 8.0, 2.0, 7.0], dtype=np.float64)
    expected = pd.Series(values).shift(2).to_numpy()
    np.testing.assert_array_equal(Lag(values, timeperiod=2).compute(), expected)

    state = Lag([], timeperiod=2)
    assert state.extend(values[:3]) is state
    assert state.extend(values[3:]) is state
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for value in values:
        assert state.append(float(value)) is state
    np.testing.assert_array_equal(state.compute(), expected)
