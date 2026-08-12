from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats
from taflow.metrics.treynor_ratio import TreynorRatio
PERFORMANCE_ANALYTICS_2_1_0_SHA256 = 'fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d'

def _performance_analytics_source_convention(returns: np.ndarray, benchmark_returns: np.ndarray, periods_per_year: float, annual_risk_free_rate: float) -> float | None:
    """Translate the pinned CRAN source formula for specification parity."""
    usable = ~(np.isnan(returns) | np.isnan(benchmark_returns))
    primary = returns[usable]
    benchmark = benchmark_returns[usable]
    if len(primary) < 2:
        return None
    period_risk_free_rate = np.expm1(np.log1p(annual_risk_free_rate) / periods_per_year)
    primary_excess = primary - period_risk_free_rate
    benchmark_excess = benchmark - period_risk_free_rate
    covariance = np.cov(primary_excess, benchmark_excess, ddof=1)[0, 1]
    benchmark_variance = np.var(benchmark_excess, ddof=1)
    beta = covariance / benchmark_variance
    if beta == 0.0 or not np.isfinite(beta):
        return None
    annualized_excess = np.prod(1.0 + primary_excess) ** (periods_per_year / len(primary)) - 1.0
    result = annualized_excess / beta
    return float(result) if np.isfinite(result) else None

def test_treynor_ratio_matches_quantstats_executable_cross_check() -> None:
    """At rf=0 and periods=n, QuantStats is algebraically the same oracle."""
    returns = pd.Series([0.08, -0.03, 0.04, 0.01, -0.02])
    benchmark = pd.Series([0.05, -0.02, 0.02, 0.0, -0.01])
    periods = float(len(returns))
    expected = float(quantstats.stats.treynor_ratio(returns, benchmark, periods=periods, rf=0.0))
    actual = TreynorRatio(periods_per_year=periods).from_returns(returns, benchmark).compute()
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

@pytest.mark.parametrize(('returns', 'benchmark', 'periods_per_year', 'annual_risk_free_rate'), [(np.array([0.08, -0.03, 0.04, 0.01, -0.02]), np.array([0.05, -0.02, 0.02, 0.0, -0.01]), 12.0, 0.06167781186449828), (np.array([np.nan, 0.02, -0.03, 0.01, 0.04]), np.array([0.01, np.nan, -0.01, 0.005, 0.02]), 52.0, -0.01), (np.random.default_rng(20260811).normal(0.0004, 0.012, 513), np.random.default_rng(20260812).normal(0.0003, 0.009, 513), 365.0, 0.05)])
def test_treynor_ratio_matches_pinned_performanceanalytics_source_convention(returns: np.ndarray, benchmark: np.ndarray, periods_per_year: float, annual_risk_free_rate: float) -> None:
    assert len(PERFORMANCE_ANALYTICS_2_1_0_SHA256) == 64
    expected = _performance_analytics_source_convention(returns, benchmark, periods_per_year, annual_risk_free_rate)
    actual = TreynorRatio(periods_per_year=periods_per_year, annual_risk_free_rate=annual_risk_free_rate).from_returns(returns, benchmark).compute()
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-11, abs=1e-13)

def test_treynor_ratio_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05])
    benchmark_returns = np.array([0.02, -0.1, 0.01])
    settings = {'periods_per_year': 12.0, 'annual_risk_free_rate': 0.04}
    expected = TreynorRatio(**settings).from_returns(returns, benchmark_returns).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    benchmark_equity = np.array([200.0, 204.0, 183.6, 185.436])
    assert TreynorRatio(**settings).from_equity(equity, benchmark_equity).compute() == pytest.approx(expected)
    assert TreynorRatio(**settings).from_pnl(np.array([10.0, -22.0, 4.4]), np.array([4.0, -20.4, 1.836]), initial_capital=100.0, benchmark_initial_capital=200.0).compute() == pytest.approx(expected)
    assert TreynorRatio(**settings).from_log_returns(np.log1p(returns), np.log1p(benchmark_returns)).compute() == pytest.approx(expected)
    state = TreynorRatio(**settings).from_returns([], [])
    assert state.value is None
    assert state.append(returns[0], benchmark_returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:], benchmark_returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns, benchmark_returns).compute() == pytest.approx(expected)

def test_treynor_ratio_pairwise_missing_minimum_and_zero_beta() -> None:
    returns = np.array([np.nan, 0.01, 0.05, -0.02, np.nan])
    benchmark = np.array([0.02, 0.0, np.nan, -0.01, np.nan])
    state = TreynorRatio(periods_per_year=1.0).from_returns(returns, benchmark)
    assert len(state) == 2
    assert state.compute() == pytest.approx(_performance_analytics_source_convention(returns, benchmark, 1.0, 0.0))
    assert TreynorRatio().from_returns([0.25], [0.1]).compute() is None
    assert TreynorRatio().from_returns([0.01, 0.01], [0.0, 0.02]).compute() is None
    assert TreynorRatio().from_returns([0.01, 0.02], [0.1, 0.1]).compute() is None

def test_treynor_ratio_rejects_invalid_input_without_partial_mutation() -> None:
    state = TreynorRatio().from_returns([0.01, 0.02], [0.0, 0.01])
    original_value = state.value
    original_length = len(state)
    with pytest.raises(ValueError, match='equal length'):
        state.extend([0.03, 0.04], [0.02])
    assert len(state) == original_length
    assert state.value == original_value
    with pytest.raises(ValueError):
        TreynorRatio(nan_policy='raise').from_returns([np.nan], [0.0])
    with pytest.raises(ValueError):
        TreynorRatio().from_returns([np.inf], [0.0])
    with pytest.raises(ValueError):
        TreynorRatio().from_returns([0.01, 0.02], [0.01])
    with pytest.raises(ValueError):
        TreynorRatio(periods_per_year=0.0).from_returns([0.01, 0.02], [0.0, 0.01])
    with pytest.raises(ValueError):
        TreynorRatio(annual_risk_free_rate=-1.0).from_returns([0.01, 0.02], [0.0, 0.01])
    unbound = TreynorRatio()
    with pytest.raises(ValueError):
        unbound.append(0.01, 0.01)
