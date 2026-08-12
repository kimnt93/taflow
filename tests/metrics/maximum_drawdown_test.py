from __future__ import annotations
import empyrical
import numpy as np
import pytest
from taflow.metrics.maximum_drawdown import MaximumDrawdown

@pytest.mark.parametrize('returns', [np.array([0.01]), np.array([-0.2]), np.array([0.1, -0.2, 0.05]), np.zeros(32), np.linspace(-0.04, 0.05, 101), np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_maximum_drawdown_matches_empyrical(returns: np.ndarray) -> None:
    actual = MaximumDrawdown().from_returns(returns).compute()
    expected = float(empyrical.max_drawdown(returns))
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_maximum_drawdown_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05, -0.25, 0.1])
    expected = MaximumDrawdown().from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert MaximumDrawdown().from_equity(equity).compute() == pytest.approx(expected)
    assert MaximumDrawdown().from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert MaximumDrawdown().from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = MaximumDrawdown().from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.extend(returns[1:3]) is state
    assert state.extend(returns[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_maximum_drawdown_uses_phantom_wealth_one() -> None:
    assert MaximumDrawdown().from_returns([-0.2]).compute() == pytest.approx(-0.2)
    assert MaximumDrawdown().from_returns([0.2]).compute() == pytest.approx(0.0)

def test_maximum_drawdown_missing_and_invalid_values() -> None:
    assert len(MaximumDrawdown().from_returns([np.nan, 0.01])) == 1
    with pytest.raises(ValueError):
        MaximumDrawdown(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        MaximumDrawdown().from_returns([np.inf])
    with pytest.raises(ValueError):
        MaximumDrawdown().from_returns([-1.01])

def test_maximum_drawdown_requires_semantic_ingestion_before_append() -> None:
    with pytest.raises(ValueError):
        MaximumDrawdown().append(0.01)
