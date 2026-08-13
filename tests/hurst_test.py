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
