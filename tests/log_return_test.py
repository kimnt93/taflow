import numpy as np
import pandas as pd

from taflow import LogReturn


def test_log_return_matches_pandas_and_lifecycle() -> None:
    values = np.array([2.0, 4.0, 1.0, 8.0, 2.0, 7.0], dtype=np.float64)
    series = pd.Series(values)
    expected = np.log(series / series.shift(2)).to_numpy()
    np.testing.assert_allclose(
        LogReturn(timeperiod=2).extend(values).compute(), expected, equal_nan=True
    )

    state = LogReturn(timeperiod=2)
    assert state.extend(values[:3]) is state
    assert state.extend(values[3:]) is state
    np.testing.assert_allclose(state.compute(), expected, equal_nan=True)
    assert state.reset() is state
    for value in values:
        assert state.append(float(value)) is state
    np.testing.assert_allclose(state.compute(), expected, equal_nan=True)
