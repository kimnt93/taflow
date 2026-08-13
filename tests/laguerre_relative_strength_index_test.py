"""Correctness and lifecycle tests for Laguerre Relative Strength Index."""

import numpy as np
import pytest

from taflow import LaguerreRelativeStrengthIndex
from tests.oracle_assertions import assert_registered_oracle_match


def test_laguerre_relative_strength_index_matches_registered_wickra_oracle() -> None:
    """Compare random and adversarial histories through the public class."""
    assert_registered_oracle_match("LaguerreRelativeStrengthIndex")


def test_lifecycle_is_bitwise_invariant() -> None:
    rng = np.random.default_rng(52519)
    close = 100.0 + rng.normal(size=431).cumsum()
    batch = LaguerreRelativeStrengthIndex(0.35).extend(close)
    chunked = LaguerreRelativeStrengthIndex(0.35)
    chunked.extend(close[:53]).extend(close[53:])
    np.testing.assert_array_equal(chunked.compute(), batch.compute())

    chunked.reset()
    for value in close:
        chunked.append(float(value))
    np.testing.assert_array_equal(chunked.compute(), batch.compute())
    assert len(chunked) == len(close)


def test_neutral_value_and_validation() -> None:
    state = LaguerreRelativeStrengthIndex(0.0)
    assert state.append(42.0).value == 50.0
    assert LaguerreRelativeStrengthIndex(1.0).value is None
    for invalid_gamma in (-0.1, 1.1, np.nan, np.inf):
        with pytest.raises(ValueError):
            LaguerreRelativeStrengthIndex(invalid_gamma)
