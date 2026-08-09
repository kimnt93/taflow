import numpy as np
import talib

from taflow import TripleExponentialAverage


def test_matches_talib_t3() -> None:
    values = 100.0 + np.sin(np.arange(180) * 0.11).cumsum()
    expected = talib.T3(values, timeperiod=7, vfactor=0.7)
    actual = TripleExponentialAverage(values, 7, 0.7).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=1e-11, equal_nan=True)


def test_lifecycle_is_chunk_invariant() -> None:
    values = np.linspace(10.0, 20.0, 100)
    expected = TripleExponentialAverage(values, 7, 0.7).compute()
    state = TripleExponentialAverage(np.array([], dtype=np.float64), 7, 0.7)
    state.extend(values[:27]).extend(values[27:])
    np.testing.assert_array_equal(state.compute(), expected)
    state.reset().extend(values)
    np.testing.assert_array_equal(state.compute(), expected)
