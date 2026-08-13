"""Correctness and lifecycle tests for Variable Index Dynamic Average."""

import numpy as np
import pytest

from taflow import VariableIndexDynamicAverage
from tests.oracle_assertions import assert_registered_oracle_match


def test_variable_index_dynamic_average_matches_registered_wickra_oracle() -> None:
    """Compare random and adversarial histories through the public class."""
    assert_registered_oracle_match("VariableIndexDynamicAverage")


def test_lifecycle_is_bitwise_invariant_with_custom_parameters() -> None:
    rng = np.random.default_rng(94117)
    close = 100.0 + rng.normal(size=431).cumsum()
    batch = VariableIndexDynamicAverage(17, 7, 0.35).extend(close)

    chunked = VariableIndexDynamicAverage(17, 7, 0.35)
    assert chunked.extend(close[:53]) is chunked
    assert chunked.extend(close[53:]) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())
    assert chunked.value == batch.value

    assert chunked.reset() is chunked
    for value in close:
        assert chunked.append(float(value)) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())
    assert len(chunked) == len(close)


def test_warm_up_and_validation() -> None:
    state = VariableIndexDynamicAverage(length=3, cmo_period=3)
    state.append(1.0).append(2.0).append(3.0)
    assert state.value is None
    assert np.isnan(state.compute()[-1])
    state.append(4.0)
    assert state.value == 4.0

    with pytest.raises(ValueError):
        VariableIndexDynamicAverage().extend(None)
    with pytest.raises(ValueError):
        VariableIndexDynamicAverage(length=0)
    with pytest.raises(ValueError):
        VariableIndexDynamicAverage(cmo_period=0)
    for invalid_alpha in (0.0, -0.1, 1.1, np.nan, np.inf):
        with pytest.raises(ValueError):
            VariableIndexDynamicAverage(14, 9, invalid_alpha)
