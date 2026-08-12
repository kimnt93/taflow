from __future__ import annotations
import empyrical
import numpy as np
import pytest
from taflow.metrics.down_market_capture_ratio import DownMarketCaptureRatio

def _empyrical_down_capture(returns: np.ndarray, benchmark_returns: np.ndarray) -> float | None:
    usable = ~(np.isnan(returns) | np.isnan(benchmark_returns))
    primary = returns[usable]
    benchmark = benchmark_returns[usable]
    expected = float(empyrical.down_capture(primary, benchmark, period='daily'))
    return expected if np.isfinite(expected) else None

@pytest.mark.parametrize(('returns', 'benchmark_returns'), [(np.array([0.01]), np.array([-0.02])), (np.array([0.1, -0.2, 0.05, 0.01]), np.array([0.02, -0.1, -0.01, 0.03])), (np.array([np.nan, 0.02, -0.03, 0.01, np.nan]), np.array([-0.01, np.nan, -0.02, 0.005, np.nan])), (np.linspace(-0.004, 0.005, 101), np.linspace(-0.005, 0.004, 101)), (np.random.default_rng(20260821).normal(0.0004, 0.012, 513), np.random.default_rng(20260822).normal(0.0003, 0.009, 513))])
def test_down_market_capture_ratio_matches_empyrical(returns: np.ndarray, benchmark_returns: np.ndarray) -> None:
    actual = DownMarketCaptureRatio().from_returns(returns, benchmark_returns).compute()
    expected = _empyrical_down_capture(returns, benchmark_returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-11, abs=1e-13)

def test_down_market_capture_ratio_uses_explicit_annualization() -> None:
    returns = np.array([0.02, -0.01, 0.03, 0.005])
    benchmark = np.array([-0.01, -0.005, 0.015, -0.002])
    selected = benchmark < 0.0
    primary_cagr = float(empyrical.annual_return(returns[selected], annualization=12.0))
    benchmark_cagr = float(empyrical.annual_return(benchmark[selected], annualization=12.0))
    actual = DownMarketCaptureRatio(periods_per_year=12.0).from_returns(returns, benchmark).compute()
    assert actual == pytest.approx(primary_cagr / benchmark_cagr, rel=1e-12)

def test_down_market_capture_ratio_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05])
    benchmark_returns = np.array([0.02, -0.1, -0.01])
    expected = DownMarketCaptureRatio(periods_per_year=12.0).from_returns(returns, benchmark_returns).compute()
    assert DownMarketCaptureRatio(periods_per_year=12.0).from_equity(np.array([100.0, 110.0, 88.0, 92.4]), np.array([200.0, 204.0, 183.6, 181.764])).compute() == pytest.approx(expected)
    assert DownMarketCaptureRatio(periods_per_year=12.0).from_pnl(np.array([10.0, -22.0, 4.4]), np.array([4.0, -20.4, -1.836]), initial_capital=100.0, benchmark_initial_capital=200.0).compute() == pytest.approx(expected)
    assert DownMarketCaptureRatio(periods_per_year=12.0).from_log_returns(np.log1p(returns), np.log1p(benchmark_returns)).compute() == pytest.approx(expected)
    state = DownMarketCaptureRatio(periods_per_year=12.0).from_returns([], [])
    assert state.value is None
    assert state.append(returns[0], benchmark_returns[0]) is state
    assert state.extend(returns[1:], benchmark_returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns, benchmark_returns).compute() == pytest.approx(expected)

def test_down_market_capture_ratio_pairwise_missing_and_minimum() -> None:
    returns = np.array([np.nan, 0.01, 0.05, -0.02, np.nan])
    benchmark = np.array([-0.02, 0.0, np.nan, -0.01, np.nan])
    actual = DownMarketCaptureRatio().from_returns(returns, benchmark)
    assert len(actual) == 2
    assert actual.compute() == pytest.approx(_empyrical_down_capture(returns, benchmark))
    assert DownMarketCaptureRatio().from_returns([0.25], [-0.1]).compute() is not None
    assert DownMarketCaptureRatio().from_returns([0.01, -0.02], [0.0, 0.02]).compute() is None

def test_down_market_capture_ratio_rejects_misalignment_without_mutation() -> None:
    state = DownMarketCaptureRatio().from_returns([0.01, 0.02], [-0.03, 0.01])
    original_value = state.value
    original_length = len(state)
    with pytest.raises(ValueError, match='equal length'):
        state.extend([0.03, 0.04], [-0.02])
    assert len(state) == original_length
    assert state.value == original_value

def test_down_market_capture_ratio_invalid_and_input_method_contracts() -> None:
    with pytest.raises(ValueError):
        DownMarketCaptureRatio(nan_policy='raise').from_returns([np.nan], [-0.01])
    with pytest.raises(ValueError):
        DownMarketCaptureRatio().from_returns([np.inf], [-0.01])
    with pytest.raises(ValueError):
        DownMarketCaptureRatio().from_returns([0.01, 0.02], [-0.01])
    with pytest.raises(ValueError):
        DownMarketCaptureRatio(periods_per_year=0.0).from_returns([0.01], [-0.02])
    unbound = DownMarketCaptureRatio()
    with pytest.raises(ValueError):
        unbound.append(0.01, 0.01)
