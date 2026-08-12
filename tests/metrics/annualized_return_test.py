"""Correctness and lifecycle tests for AnnualizedReturn."""
from __future__ import annotations
import empyrical
import numpy as np
import pytest
from taflow.metrics import AnnualizedReturn

@pytest.mark.parametrize('periods_per_year', [1.0, 12.0, 52.0, 252.0, 365.0])
@pytest.mark.parametrize('returns', [np.array([0.02]), np.array([0.01, -0.02, 0.03, 0.0, -0.01]), np.zeros(32), np.linspace(-0.02, 0.03, 64), np.random.default_rng(9721).normal(0.0004, 0.012, 257), np.array([np.nan, 0.01, -0.02, np.nan, 0.015])])
def test_matches_empyrical_annual_return(returns: np.ndarray, periods_per_year: float) -> None:
    actual = AnnualizedReturn(periods_per_year=periods_per_year).from_returns(returns).compute()
    oracle_returns = returns[~np.isnan(returns)]
    expected = empyrical.annual_return(oracle_returns, annualization=periods_per_year)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05, 0.03], dtype=np.float64)
    equity = 100.0 * np.cumprod(np.r_[1.0, 1.0 + returns])
    pnl = np.diff(equity)
    log_returns = np.log1p(returns)
    expected = empyrical.annual_return(returns, annualization=12.0)
    states = [AnnualizedReturn(periods_per_year=12.0).from_returns([]), AnnualizedReturn(periods_per_year=12.0).from_log_returns(log_returns), AnnualizedReturn(periods_per_year=12.0).from_equity(equity), AnnualizedReturn(periods_per_year=12.0).from_pnl(pnl, initial_capital=100.0)]
    states[0].append(returns[0]).extend(returns[1:3]).append(returns[3])
    for state in states:
        assert state.compute() == pytest.approx(expected, rel=1e-12, abs=1e-14)
        assert state.value == state.compute()
        assert len(state) == returns.size
    streamed = states[0]
    assert streamed.reset() is streamed
    assert streamed.value is None
    assert streamed.compute() is None
    assert len(streamed) == 0
    assert streamed.extend(returns) is streamed
    assert streamed.compute() == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_nan_policy_and_edge_contract() -> None:
    omitted = AnnualizedReturn(periods_per_year=252.0).from_returns([np.nan, 0.25, -1.0])
    assert omitted.compute() == -1.0
    assert len(omitted) == 2
    with pytest.raises(ValueError, match='NaN'):
        AnnualizedReturn(nan_policy='raise').from_returns([0.01, np.nan])
    with pytest.raises(ValueError, match='periods_per_year'):
        AnnualizedReturn(periods_per_year=0.0).from_returns([])
    with pytest.raises(ValueError, match='simple returns'):
        AnnualizedReturn().from_returns([-1.01])
    with pytest.raises(ValueError, match='initial_capital'):
        AnnualizedReturn().from_pnl([1.0], initial_capital=0.0)
