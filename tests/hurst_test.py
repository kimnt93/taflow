import numpy as np

from taflow import Hurst
from tests.oracle_assertions import assert_registered_oracle_match


def test_hurst_lifecycle_and_reset():
    state = Hurst(timeperiod=8, chunks=4)
    state.extend(np.arange(1.0, 9.0))
    assert np.isfinite(state.compute()[-1])
    assert len(state) == 8
    state.reset()
    assert state.value is None


def test_hurst_matches_wickra() -> None:
    assert_registered_oracle_match("Hurst")


def test_hurst_bulk_chunks_and_scalar_lifecycle_match() -> None:
    values = 100.0 + np.random.default_rng(719).normal(size=173).cumsum()
    all_at_once = Hurst(timeperiod=20, chunks=4).extend(values)

    chunked = Hurst(timeperiod=20, chunks=4)
    for chunk in np.array_split(values, 17):
        chunked.extend(chunk)
    np.testing.assert_array_equal(chunked.compute(), all_at_once.compute())
    assert chunked.value == all_at_once.value

    scalar = Hurst(timeperiod=20, chunks=4)
    for value in values:
        scalar.append(value)
    np.testing.assert_array_equal(scalar.compute(), all_at_once.compute())
    assert scalar.value == all_at_once.value

    scalar.reset().extend(values)
    np.testing.assert_array_equal(scalar.compute(), all_at_once.compute())
