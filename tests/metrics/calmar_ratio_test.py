"""External correctness and lifecycle tests for CalmarRatio."""
from __future__ import annotations
import empyrical
import numpy as np
import pytest
from taflow.metrics.calmar_ratio import CalmarRatio

@pytest.mark.parametrize('periods_per_year', [1.0, 12.0, 52.0, 252.0, 365.0])
@pytest.mark.parametrize('returns', [np.array([-0.02]), np.array([0.1, -0.2, 0.05]), np.linspace(0.03, -0.04, 101), np.resize(np.array([0.12, 0.0, -0.08, -0.08, 0.12]), 125), np.random.default_rng(20260811).normal(0.0004, 0.012, 513), np.array([np.nan, 0.02, -0.03, np.nan, 0.01])])
def test_calmar_ratio_matches_empyrical(returns: np.ndarray, periods_per_year: float) -> None:
    actual = CalmarRatio(periods_per_year=periods_per_year).from_returns(returns).compute()
    oracle_returns = returns[~np.isnan(returns)]
    expected = float(empyrical.calmar_ratio(oracle_returns, annualization=periods_per_year))
    assert np.isfinite(expected)
    assert actual == pytest.approx(expected, rel=1e-11, abs=1e-13)

def test_calmar_ratio_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05, -0.25, 0.1], dtype=np.float64)
    settings = {'periods_per_year': 12.0}
    expected = CalmarRatio(**settings).from_returns(returns).compute()
    equity = 100.0 * np.cumprod(np.r_[1.0, 1.0 + returns])
    pnl = np.diff(equity)
    assert CalmarRatio(**settings).from_equity(equity).compute() == pytest.approx(expected)
    assert CalmarRatio(**settings).from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert CalmarRatio(**settings).from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = CalmarRatio(**settings).from_returns([])
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

def test_calmar_ratio_undefined_missing_and_invalid_contract() -> None:
    assert CalmarRatio().from_returns([]).compute() is None
    assert CalmarRatio().from_returns([0.1, 0.0, 0.2]).compute() is None
    assert CalmarRatio().from_returns([0.0, 0.0]).compute() is None
    assert CalmarRatio().from_returns([-1.0]).compute() == pytest.approx(-1.0)
    assert len(CalmarRatio().from_returns([np.nan, -0.01, np.nan])) == 1
    with pytest.raises(ValueError, match='NaN'):
        CalmarRatio(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        CalmarRatio().from_returns([np.inf])
    with pytest.raises(ValueError):
        CalmarRatio().from_returns([-1.01])
    with pytest.raises(ValueError, match='periods_per_year'):
        CalmarRatio(periods_per_year=0.0).from_returns([])
    with pytest.raises(ValueError, match='initial_capital'):
        CalmarRatio().from_pnl([1.0], initial_capital=0.0)

def test_calmar_ratio_requires_semantic_ingestion_before_append() -> None:
    with pytest.raises(ValueError):
        CalmarRatio().append(0.01)
