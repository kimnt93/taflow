from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as quantstats_stats
from taflow.metrics.coefficient_of_determination import CoefficientOfDetermination

def _usable_pairs(returns: np.ndarray, benchmark_returns: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    mask = ~(np.isnan(returns) | np.isnan(benchmark_returns))
    return (returns[mask], benchmark_returns[mask])

def _quantstats_coefficient(returns: np.ndarray, benchmark_returns: np.ndarray) -> float | None:
    returns, benchmark_returns = _usable_pairs(returns, benchmark_returns)
    if returns.size < 2 or np.ptp(returns) == 0.0 or np.ptp(benchmark_returns) == 0.0:
        return None
    expected = float(quantstats_stats.r_squared(pd.Series(returns), pd.Series(benchmark_returns), prepare_returns=False))
    return expected if np.isfinite(expected) else None

@pytest.mark.parametrize(('returns', 'benchmark_returns'), [(np.array([0.01, -0.01]), np.array([0.02, -0.02])), (np.array([0.1, -0.2, 0.05, 0.01]), np.array([0.02, -0.1, 0.01, 0.03])), (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.array([0.01, np.nan, -0.01, 0.02, 0.005])), (np.linspace(-0.04, 0.05, 101), np.linspace(-0.02, 0.04, 101) ** 3), (np.random.default_rng(20260811).normal(0.0004, 0.012, 513), np.random.default_rng(20260812).normal(0.0003, 0.009, 513))])
def test_coefficient_of_determination_matches_quantstats_and_numpy(returns: np.ndarray, benchmark_returns: np.ndarray) -> None:
    actual = CoefficientOfDetermination().from_returns(returns, benchmark_returns).compute()
    expected = _quantstats_coefficient(returns, benchmark_returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)
    usable_returns, usable_benchmark = _usable_pairs(returns, benchmark_returns)
    numpy_expected = float(np.corrcoef(usable_returns, usable_benchmark)[0, 1] ** 2)
    assert actual == pytest.approx(numpy_expected, rel=1e-12, abs=1e-14)

def test_coefficient_of_determination_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05])
    benchmark_returns = np.array([0.02, -0.1, 0.01])
    expected = CoefficientOfDetermination().from_returns(returns, benchmark_returns).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    benchmark_equity = np.array([200.0, 204.0, 183.6, 185.436])
    assert CoefficientOfDetermination().from_equity(equity, benchmark_equity).compute() == pytest.approx(expected)
    assert CoefficientOfDetermination().from_pnl(np.array([10.0, -22.0, 4.4]), np.array([4.0, -20.4, 1.836]), initial_capital=100.0, benchmark_initial_capital=200.0).compute() == pytest.approx(expected)
    assert CoefficientOfDetermination().from_log_returns(np.log1p(returns), np.log1p(benchmark_returns)).compute() == pytest.approx(expected)
    state = CoefficientOfDetermination().from_returns([], [])
    assert state.value is None
    assert state.append(returns[0], benchmark_returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:], benchmark_returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns, benchmark_returns).compute() == pytest.approx(expected)

def test_coefficient_of_determination_pairwise_missing_and_undefined_cases() -> None:
    returns = np.array([np.nan, 0.01, 0.05, -0.02, np.nan])
    benchmark = np.array([0.02, 0.0, np.nan, -0.01, np.nan])
    actual = CoefficientOfDetermination().from_returns(returns, benchmark)
    assert len(actual) == 2
    assert actual.compute() == pytest.approx(_quantstats_coefficient(returns, benchmark))
    assert CoefficientOfDetermination().from_returns([0.25], [0.1]).compute() is None
    assert CoefficientOfDetermination().from_returns([0.01, 0.01], [0.1, 0.2]).compute() is None
    assert CoefficientOfDetermination().from_returns([0.01, 0.02], [0.1, 0.1]).compute() is None

def test_coefficient_of_determination_rejects_misalignment_without_mutation() -> None:
    state = CoefficientOfDetermination().from_returns([0.01, 0.02], [0.0, 0.01])
    original_value = state.value
    original_length = len(state)
    with pytest.raises(ValueError, match='equal length'):
        state.extend([0.03, 0.04], [0.02])
    assert len(state) == original_length
    assert state.value == original_value

def test_coefficient_of_determination_validation_and_input_method_contracts() -> None:
    with pytest.raises(ValueError):
        CoefficientOfDetermination(nan_policy='raise').from_returns([np.nan], [0.0])
    with pytest.raises(ValueError):
        CoefficientOfDetermination().from_returns([np.inf], [0.0])
    with pytest.raises(ValueError):
        CoefficientOfDetermination().from_returns([0.01, 0.02], [0.01])
    unbound = CoefficientOfDetermination()
    with pytest.raises(ValueError):
        unbound.append(0.01, 0.01)
