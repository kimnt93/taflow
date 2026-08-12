from __future__ import annotations
import empyrical
import numpy as np
import pytest
from taflow.metrics.total_return import TotalReturn

@pytest.mark.parametrize('returns', [np.array([0.01]), np.array([0.1, -0.2, 0.05]), np.zeros(32), np.linspace(-0.04, 0.05, 101), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_total_return_matches_empyrical(returns: np.ndarray) -> None:
    actual = TotalReturn().from_returns(returns).compute()
    expected = float(empyrical.cum_returns_final(returns))
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_total_return_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05])
    expected = TotalReturn().from_returns(returns).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    pnl = np.array([10.0, -22.0, 4.4])
    assert TotalReturn().from_equity(equity).compute() == pytest.approx(expected)
    assert TotalReturn().from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert TotalReturn().from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = TotalReturn().from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_total_return_missing_and_invalid_values() -> None:
    assert len(TotalReturn().from_returns([np.nan, 0.01])) == 1
    with pytest.raises(ValueError):
        TotalReturn(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        TotalReturn().from_returns([np.inf])
    with pytest.raises(ValueError):
        TotalReturn().from_returns([-1.01])

def test_total_return_requires_semantic_ingestion_before_append() -> None:
    with pytest.raises(ValueError):
        TotalReturn().append(0.01)
