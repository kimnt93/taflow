"""External correctness and lifecycle tests for RecoveryFactor."""

from __future__ import annotations

import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs

from taflow.metrics.recovery_factor import RecoveryFactor


def quantstats_recovery_factor(returns: np.ndarray) -> float:
    """Evaluate the pinned oracle without its top-level preparation heuristic."""
    series = pd.Series(
        returns,
        index=pd.date_range("2000-01-01", periods=len(returns), freq="D"),
    )
    return float(qs.recovery_factor(series, rf=0.0, prepare_returns=False))


@pytest.mark.parametrize(
    "returns",
    [
        np.array([-0.02]),
        np.array([0.10, -0.20, 0.05]),
        np.linspace(0.03, -0.04, 101),
        np.resize(np.array([0.12, 0.0, -0.08, -0.08, 0.12]), 125),
        np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
        np.array([np.nan, 0.02, -0.03, np.nan, 0.01]),
    ],
)
def test_recovery_factor_matches_quantstats(returns: np.ndarray) -> None:
    actual = RecoveryFactor.from_returns(returns).compute()
    expected = quantstats_recovery_factor(returns)
    assert np.isfinite(expected)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_recovery_factor_freezes_arithmetic_sum_not_compounding() -> None:
    returns = np.array([0.10, -0.10, 0.10])
    actual = RecoveryFactor.from_returns(returns).compute()
    arithmetic_numerator = abs(float(returns.sum()))
    compounded_numerator = abs(float(np.prod(1.0 + returns) - 1.0))

    assert actual == pytest.approx(1.0)
    assert arithmetic_numerator == pytest.approx(0.10)
    assert compounded_numerator == pytest.approx(0.089)
    assert actual == pytest.approx(quantstats_recovery_factor(returns))


def test_recovery_factor_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05, -0.25, 0.10], dtype=np.float64)
    expected = RecoveryFactor.from_returns(returns).compute()
    equity = 100.0 * np.cumprod(np.r_[1.0, 1.0 + returns])
    pnl = np.diff(equity)

    assert RecoveryFactor.from_equity(equity).compute() == pytest.approx(expected)
    assert RecoveryFactor.from_pnl(
        pnl, initial_equity=100.0
    ).compute() == pytest.approx(expected)
    assert RecoveryFactor.from_log_returns(np.log1p(returns)).compute() == pytest.approx(
        expected
    )

    state = RecoveryFactor.from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:3]) is state
    assert state.extend(returns[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == returns.size

    assert state.reset() is state
    assert state.value is None
    assert len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_recovery_factor_undefined_missing_and_invalid_contract() -> None:
    assert RecoveryFactor.from_returns([]).compute() is None
    assert RecoveryFactor.from_returns([0.10, 0.0, 0.20]).compute() is None
    assert RecoveryFactor.from_returns([-0.10, 0.10]).compute() == pytest.approx(0.0)
    assert len(RecoveryFactor.from_returns([np.nan, -0.01, np.nan])) == 1

    with pytest.raises(ValueError, match="NaN"):
        RecoveryFactor.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        RecoveryFactor.from_returns([np.inf])
    with pytest.raises(ValueError):
        RecoveryFactor.from_returns([-1.01])
    with pytest.raises(ValueError, match="initial_equity"):
        RecoveryFactor.from_pnl([1.0], initial_equity=0.0)


def test_recovery_factor_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        RecoveryFactor()
