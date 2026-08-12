from __future__ import annotations

import numpy as np
import pytest
from scipy import stats

from taflow.metrics.parametric_value_at_risk import ParametricValueAtRisk


@pytest.mark.parametrize(
    ("returns", "cutoff"),
    [
        (np.array([0.01, -0.01]), 0.05),
        (np.array([0.10, -0.20, 0.05]), 0.01),
        (np.zeros(32), 0.10),
        (np.linspace(-0.04, 0.05, 101), 0.25),
        (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 0.05),
        (np.random.default_rng(20260812).normal(0.0004, 0.012, 513), 0.025),
    ],
)
def test_parametric_value_at_risk_matches_scipy(
    returns: np.ndarray, cutoff: float
) -> None:
    usable = returns[~np.isnan(returns)]
    expected = float(np.mean(usable) + stats.norm.ppf(cutoff) * np.std(usable, ddof=1))
    actual = ParametricValueAtRisk.from_returns(returns, cutoff=cutoff).compute()
    assert actual == pytest.approx(expected, rel=2e-9, abs=2e-11)


def test_parametric_value_at_risk_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05, -0.03])
    expected = ParametricValueAtRisk.from_returns(returns, cutoff=0.025).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)

    assert ParametricValueAtRisk.from_equity(
        equity, cutoff=0.025
    ).compute() == pytest.approx(expected)
    assert ParametricValueAtRisk.from_pnl(
        pnl, initial_equity=100.0, cutoff=0.025
    ).compute() == pytest.approx(expected)
    assert ParametricValueAtRisk.from_log_returns(
        np.log1p(returns), cutoff=0.025
    ).compute() == pytest.approx(expected)

    state = ParametricValueAtRisk.from_returns([], cutoff=0.025)
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    before_continuation = state.compute()
    assert state.compute() == before_continuation
    assert state.append(0.02) is state
    expected_continuation = ParametricValueAtRisk.from_returns(
        np.r_[returns, 0.02], cutoff=0.025
    ).compute()
    assert state.compute() == pytest.approx(expected_continuation)
    assert state.reset() is state and len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_parametric_value_at_risk_constant_missing_and_validation() -> None:
    assert ParametricValueAtRisk.from_returns([]).compute() is None
    assert ParametricValueAtRisk.from_returns([0.01]).compute() is None
    assert ParametricValueAtRisk.from_returns([0.0125, 0.0125]).compute() == pytest.approx(
        0.0125
    )
    assert len(ParametricValueAtRisk.from_returns([np.nan, 0.02, -0.01])) == 2
    with pytest.raises(ValueError):
        ParametricValueAtRisk.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        ParametricValueAtRisk.from_returns([np.inf])
    with pytest.raises(ValueError):
        ParametricValueAtRisk.from_returns([-1.01])
    for cutoff in [0.0, 1.0, np.nan, np.inf]:
        with pytest.raises(ValueError):
            ParametricValueAtRisk.from_returns([0.01, -0.01], cutoff=cutoff)


def test_parametric_value_at_risk_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        ParametricValueAtRisk()
