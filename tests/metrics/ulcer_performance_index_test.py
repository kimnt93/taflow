"""External correctness and lifecycle tests for UlcerPerformanceIndex."""
from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.ulcer_performance_index import UlcerPerformanceIndex

def quantstats_ulcer_performance_index(returns: np.ndarray) -> float:
    """Evaluate the pinned oracle with zero whole-sample risk-free return."""
    filtered = returns[~np.isnan(returns)]
    series = pd.Series(filtered, index=pd.date_range('2000-01-01', periods=len(filtered), freq='D'))
    return float(qs.ulcer_performance_index(series, rf=0.0))

@pytest.mark.parametrize('returns', [np.array([0.1, -0.2]), np.linspace(-0.04, 0.05, 101), np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), np.array([0.2, -0.2, 0.25, -0.2, 0.25, -0.2]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513), np.array([np.nan, 0.02, -0.03, np.nan, 0.01])])
def test_ulcer_performance_index_matches_quantstats(returns: np.ndarray) -> None:
    actual = UlcerPerformanceIndex().from_returns(returns).compute()
    expected = quantstats_ulcer_performance_index(returns)
    assert np.isfinite(expected)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_ulcer_performance_index_freezes_compounded_unannualized_numerator() -> None:
    returns = np.array([0.1, -0.1, 0.1])
    actual = UlcerPerformanceIndex().from_returns(returns).compute()
    compounded_numerator = float(np.prod(1.0 + returns) - 1.0)
    arithmetic_numerator = float(returns.sum())
    assert compounded_numerator == pytest.approx(0.089)
    assert arithmetic_numerator == pytest.approx(0.1)
    assert actual == pytest.approx(quantstats_ulcer_performance_index(returns))

def test_ulcer_performance_index_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05, -0.25, 0.1], dtype=np.float64)
    expected = UlcerPerformanceIndex().from_returns(returns).compute()
    equity = 100.0 * np.cumprod(np.r_[1.0, 1.0 + returns])
    pnl = np.diff(equity)
    assert UlcerPerformanceIndex().from_equity(equity).compute() == pytest.approx(expected)
    assert UlcerPerformanceIndex().from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert UlcerPerformanceIndex().from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = UlcerPerformanceIndex().from_returns([])
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

def test_ulcer_performance_index_warmup_zero_risk_and_invalid_contract() -> None:
    assert UlcerPerformanceIndex().from_returns([]).compute() is None
    assert UlcerPerformanceIndex().from_returns([-0.1]).compute() is None
    assert UlcerPerformanceIndex().from_returns([0.1, 0.0, 0.2]).compute() is None
    assert len(UlcerPerformanceIndex().from_returns([np.nan, -0.01, -0.01])) == 2
    negative = UlcerPerformanceIndex().from_returns([-0.1, -0.1]).compute()
    assert negative is not None and negative < 0.0
    with pytest.raises(ValueError, match='NaN'):
        UlcerPerformanceIndex(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        UlcerPerformanceIndex().from_returns([np.inf])
    with pytest.raises(ValueError):
        UlcerPerformanceIndex().from_returns([-1.01])
    with pytest.raises(ValueError, match='initial_capital'):
        UlcerPerformanceIndex().from_pnl([1.0], initial_capital=0.0)

def test_ulcer_performance_index_requires_semantic_input_method() -> None:
    unbound = UlcerPerformanceIndex()
    with pytest.raises(ValueError):
        unbound.append(0.01)
