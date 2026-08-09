import numpy as np
import talib

from taflow import TripleExponentialMovingAverage


def test_matches_talib_tema() -> None:
    values = 100.0 + np.sin(np.arange(160) * 0.13).cumsum()
    expected = talib.TEMA(values, timeperiod=7)
    actual = TripleExponentialMovingAverage(values, 7).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=1e-12, equal_nan=True)


def test_lifecycle_is_chunk_invariant() -> None:
    values = np.linspace(10.0, 20.0, 80)
    expected = TripleExponentialMovingAverage(values, 7).compute()
    state = TripleExponentialMovingAverage(np.array([], dtype=np.float64), 7)
    state.extend(values[:23]).extend(values[23:])
    np.testing.assert_array_equal(state.compute(), expected)
    state.reset().extend(values)
    np.testing.assert_array_equal(state.compute(), expected)
