"""External correctness and lifecycle tests for WinRate."""
from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.win_rate import WinRate

def quantstats_win_rate(values: np.ndarray) -> float | None:
    """Evaluate the pinned oracle and normalize its zero-denominator sentinel."""
    filtered = values[~np.isnan(values)]
    decisive = filtered[filtered != 0.0]
    if decisive.size == 0:
        return None
    series = pd.Series(filtered, index=pd.date_range('2000-01-01', periods=len(filtered), freq='D'))
    return float(qs.win_rate(series, aggregate=None, compounded=True, prepare_returns=False))

@pytest.mark.parametrize('returns', [np.array([0.1, -0.2]), np.array([0.1, 0.0, -0.2, 0.3, 0.0]), np.linspace(-0.04, 0.05, 101), np.array([0.02, 0.02, -0.01, -0.01, 0.0, 0.02]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513), np.array([np.nan, 0.02, -0.03, np.nan, 0.0, 0.01])])
def test_win_rate_matches_quantstats(returns: np.ndarray) -> None:
    actual = WinRate().from_returns(returns).compute()
    expected = quantstats_win_rate(returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-15, abs=1e-15)

def test_win_rate_excludes_breakevens_from_quantstats_denominator() -> None:
    values = np.array([0.1, 0.0, -0.2, 0.3, 0.0])
    assert WinRate().from_returns(values).compute() == pytest.approx(2.0 / 3.0)
    assert WinRate().from_returns(values).compute() == pytest.approx(quantstats_win_rate(values))
    assert len(WinRate().from_returns(values)) == values.size

@pytest.mark.parametrize('input_method_name', ['from_pnl', 'from_trades'])
def test_win_rate_raw_pnl_and_trade_input_methods(input_method_name: str) -> None:
    values = np.array([100.0, 0.0, -25.0, 50.0, -10.0])
    input_method = getattr(WinRate(), input_method_name)
    state = input_method(values)
    assert state.compute() == pytest.approx(0.5)
    assert len(state) == values.size

def test_win_rate_lifecycle_missing_and_reset_are_invariant() -> None:
    values = np.array([0.1, np.nan, 0.0, -0.2, 0.3, 0.0])
    expected = WinRate().from_returns(values).compute()
    state = WinRate().from_returns([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value == pytest.approx(1.0)
    assert state.extend(values[1:4]) is state
    assert state.extend(values[4:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == 5
    assert state.reset() is state
    assert state.value is None
    assert len(state) == 0
    assert state.extend(values).compute() == pytest.approx(expected)

def test_win_rate_undefined_and_invalid_contract() -> None:
    assert WinRate().from_returns([]).compute() is None
    assert WinRate().from_returns([0.0, -0.0, 0.0]).compute() is None
    assert WinRate().from_returns([-0.1, -0.2]).compute() == pytest.approx(0.0)
    assert len(WinRate().from_returns([np.nan, 0.0, np.nan])) == 1
    with pytest.raises(ValueError, match='NaN'):
        WinRate(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        WinRate().from_returns([np.inf])
    with pytest.raises(ValueError):
        WinRate().from_returns([-1.01])
    with pytest.raises(TypeError):
        WinRate().from_pnl([1.0], initial_equity=100.0)

def test_win_rate_requires_semantic_input_method() -> None:
    metric = WinRate()
    with pytest.raises(ValueError):
        metric.append(0.01)
