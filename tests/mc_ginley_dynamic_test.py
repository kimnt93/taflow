import numpy as np
from taflow import McGinleyDynamic
from tests.oracle_assertions import assert_registered_oracle_match


def test_mcginley_dynamic_matches_registered_wickra_oracle() -> None:
    assert_registered_oracle_match("McGinleyDynamic")


def test_mcginley_dynamic_lifecycle() -> None:
    close = 100.0 + np.arange(64.0)
    state = McGinleyDynamic(close, length=10, c=0.6)
    first = state.compute()
    state.reset().extend(close)
    np.testing.assert_array_equal(state.compute(), first)
