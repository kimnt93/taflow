import numpy as np

from taflow import TrueStrengthIndex
from tests.oracle_assertions import assert_registered_oracle_match


def test_lifecycle() -> None:
    values = np.linspace(1.0, 20.0, 32)
    state = TrueStrengthIndex(5, 10).extend(values)
    assert len(state) == len(values)
    assert state.reset().value is None


def test_true_strength_index_matches_wickra() -> None:
    assert_registered_oracle_match("TrueStrengthIndex")
