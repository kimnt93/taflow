from __future__ import annotations
import numpy as np
import pytest
from scipy.stats import kurtosis, norm, skew
from taflow.metrics.probabilistic_sharpe_ratio import ProbabilisticSharpeRatio
VECTORBT_0_28_5_COMMIT = '993ceca7116fc8e55f4cd3a36fe43d83dab62b27'

def _vectorbt_probabilistic_sharpe(returns: np.ndarray, periods_per_year: float, annual_risk_free_rate: float, annual_benchmark_sharpe_ratio: float) -> float | None:
    """Translate vectorbt's pinned DSR kernel with an explicit benchmark SR."""
    usable = returns[~np.isnan(returns)]
    if len(usable) < 4:
        return None
    period_rf = np.expm1(np.log1p(annual_risk_free_rate) / periods_per_year)
    excess = usable - period_rf
    standard_deviation = float(np.std(excess, ddof=1))
    if standard_deviation == 0.0:
        return None
    estimated_sharpe = float(np.mean(excess) / standard_deviation)
    sample_skewness = float(skew(excess, bias=False))
    sample_kurtosis = float(kurtosis(excess, fisher=False, bias=False))
    adjustment = 1.0 - sample_skewness * estimated_sharpe + (sample_kurtosis - 1.0) * estimated_sharpe ** 2 / 4.0
    if not np.isfinite(adjustment) or adjustment <= 0.0:
        return None
    benchmark = annual_benchmark_sharpe_ratio / np.sqrt(periods_per_year)
    statistic = (estimated_sharpe - benchmark) * np.sqrt(len(usable) - 1.0) / np.sqrt(adjustment)
    return float(norm.cdf(statistic))

@pytest.mark.parametrize(('returns', 'periods_per_year', 'annual_risk_free_rate', 'benchmark'), [(np.array([0.02, -0.01, 0.03, -0.025, 0.01, -0.04, 0.015]), 252.0, 0.0, 0.5), (np.linspace(-0.04, 0.05, 101), 52.0, -0.01, -0.2), (np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), 12.0, 0.04, 1.2), (np.array([np.nan, 0.02, -0.03, np.nan, 0.01, 0.04]), 252.0, 0.03, 0.0), (np.random.default_rng(20260812).standard_t(5.0, 513) * 0.012, 365.0, 0.05, 0.8)])
def test_probabilistic_sharpe_matches_pinned_vectorbt_formula(returns: np.ndarray, periods_per_year: float, annual_risk_free_rate: float, benchmark: float) -> None:
    assert len(VECTORBT_0_28_5_COMMIT) == 40
    actual = ProbabilisticSharpeRatio(periods_per_year=periods_per_year, annual_risk_free_rate=annual_risk_free_rate, annual_benchmark_sharpe_ratio=benchmark).from_returns(returns).compute()
    expected = _vectorbt_probabilistic_sharpe(returns, periods_per_year, annual_risk_free_rate, benchmark)
    if expected is None:
        assert actual is None
    else:
        assert actual == pytest.approx(expected, rel=2e-11, abs=2e-13)

def test_probabilistic_sharpe_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05, -0.03, 0.08, -0.01])
    settings = {'periods_per_year': 12.0, 'annual_risk_free_rate': 0.03, 'annual_benchmark_sharpe_ratio': 0.4}
    expected = ProbabilisticSharpeRatio(**settings).from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert ProbabilisticSharpeRatio(**settings).from_equity(equity).compute() == pytest.approx(expected)
    assert ProbabilisticSharpeRatio(**settings).from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert ProbabilisticSharpeRatio(**settings).from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = ProbabilisticSharpeRatio(**settings).from_returns([])
    for value in returns[:3]:
        assert state.append(value) is state and state.value is None
    assert state.extend(returns[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.compute() == state.compute()
    assert state.reset() is state and len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_probabilistic_sharpe_edges_and_validation() -> None:
    assert ProbabilisticSharpeRatio().from_returns([]).compute() is None
    assert ProbabilisticSharpeRatio().from_returns([0.01, -0.01, 0.02]).compute() is None
    assert ProbabilisticSharpeRatio().from_returns([0.01] * 8).compute() is None
    assert len(ProbabilisticSharpeRatio().from_returns([np.nan, 0.02, -0.01, 0.03, 0.01])) == 4
    with pytest.raises(ValueError):
        ProbabilisticSharpeRatio(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        ProbabilisticSharpeRatio().from_returns([np.inf])
    with pytest.raises(ValueError):
        ProbabilisticSharpeRatio().from_returns([-1.01])
    with pytest.raises(ValueError):
        ProbabilisticSharpeRatio(periods_per_year=0.0).from_returns([0.01] * 4)
    with pytest.raises(ValueError):
        ProbabilisticSharpeRatio(annual_benchmark_sharpe_ratio=np.inf).from_returns([0.01] * 4)

def test_probabilistic_sharpe_requires_semantic_input_method() -> None:
    unbound = ProbabilisticSharpeRatio()
    with pytest.raises(ValueError):
        unbound.append(0.01)
