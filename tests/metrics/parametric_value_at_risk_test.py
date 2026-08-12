from __future__ import annotations
import numpy as np
import pytest
from scipy import stats
from taflow.metrics.parametric_value_at_risk import ParametricValueAtRisk

@pytest.mark.parametrize(('returns', 'cutoff'), [(np.array([0.01, -0.01]), 0.05), (np.array([0.1, -0.2, 0.05]), 0.01), (np.zeros(32), 0.1), (np.linspace(-0.04, 0.05, 101), 0.25), (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 0.05), (np.random.default_rng(20260812).normal(0.0004, 0.012, 513), 0.025)])
def test_parametric_value_at_risk_matches_scipy(returns: np.ndarray, cutoff: float) -> None:
    usable = returns[~np.isnan(returns)]
    expected = float(np.mean(usable) + stats.norm.ppf(cutoff) * np.std(usable, ddof=1))
    actual = ParametricValueAtRisk(cutoff=cutoff).from_returns(returns).compute()
    assert actual == pytest.approx(expected, rel=2e-09, abs=2e-11)

def test_parametric_value_at_risk_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05, -0.03])
    expected = ParametricValueAtRisk(cutoff=0.025).from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert ParametricValueAtRisk(cutoff=0.025).from_equity(equity).compute() == pytest.approx(expected)
    assert ParametricValueAtRisk(cutoff=0.025).from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert ParametricValueAtRisk(cutoff=0.025).from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = ParametricValueAtRisk(cutoff=0.025).from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    before_continuation = state.compute()
    assert state.compute() == before_continuation
    assert state.append(0.02) is state
    expected_continuation = ParametricValueAtRisk(cutoff=0.025).from_returns(np.r_[returns, 0.02]).compute()
    assert state.compute() == pytest.approx(expected_continuation)
    assert state.reset() is state and len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_parametric_value_at_risk_constant_missing_and_validation() -> None:
    assert ParametricValueAtRisk().from_returns([]).compute() is None
    assert ParametricValueAtRisk().from_returns([0.01]).compute() is None
    assert ParametricValueAtRisk().from_returns([0.0125, 0.0125]).compute() == pytest.approx(0.0125)
    assert len(ParametricValueAtRisk().from_returns([np.nan, 0.02, -0.01])) == 2
    with pytest.raises(ValueError):
        ParametricValueAtRisk(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        ParametricValueAtRisk().from_returns([np.inf])
    with pytest.raises(ValueError):
        ParametricValueAtRisk().from_returns([-1.01])
    for cutoff in [0.0, 1.0, np.nan, np.inf]:
        with pytest.raises(ValueError):
            ParametricValueAtRisk(cutoff=cutoff).from_returns([0.01, -0.01])

def test_parametric_value_at_risk_requires_semantic_input_method() -> None:
    metric = ParametricValueAtRisk()
    with pytest.raises(ValueError):
        metric.append(0.01)
