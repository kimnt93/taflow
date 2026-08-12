from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.profit_factor import ProfitFactor

def _quantstats_profit_factor(values: np.ndarray) -> float:
    """Evaluate QuantStats 0.0.81 without return/price preparation."""
    return float(qs.profit_factor(pd.Series(values), prepare_returns=False))

@pytest.mark.parametrize('returns', [np.array([0.1, -0.04, 0.0, 0.02, -0.01]), np.array([-0.2]), np.linspace(-0.04, 0.05, 101), np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_profit_factor_matches_quantstats(returns: np.ndarray) -> None:
    actual = ProfitFactor().from_returns(returns).compute()
    expected = _quantstats_profit_factor(returns)
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-15)

def test_profit_factor_raw_pnl_trade_input_methods_and_lifecycle() -> None:
    values = np.array([100.0, -40.0, 0.0, 20.0, -10.0])
    expected = _quantstats_profit_factor(values)
    assert ProfitFactor().from_pnl(values).compute() == pytest.approx(expected)
    assert ProfitFactor().from_trades(values).compute() == pytest.approx(expected)
    state = ProfitFactor().from_pnl([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert np.isposinf(state.value)
    assert state.extend(values[1:3]) is state
    assert state.extend(values[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(values)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == pytest.approx(expected)

def test_profit_factor_zero_denominator_normalization() -> None:
    assert ProfitFactor().from_returns([]).compute() is None
    assert ProfitFactor().from_returns([0.0, 0.0]).compute() is None
    assert np.isposinf(ProfitFactor().from_returns([0.1, 0.2]).compute())
    assert ProfitFactor().from_returns([-0.1, -0.2]).compute() == 0.0
    assert _quantstats_profit_factor(np.array([0.0, 0.0])) == 0.0

def test_profit_factor_missing_invalid_and_domain_semantics() -> None:
    state = ProfitFactor().from_returns([np.nan, 0.02, -0.01])
    assert len(state) == 2
    assert state.compute() == pytest.approx(2.0)
    with pytest.raises(ValueError):
        ProfitFactor(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        ProfitFactor().from_returns([np.inf])
    with pytest.raises(ValueError):
        ProfitFactor().from_returns([-2.0])
    assert ProfitFactor().from_pnl([-2.0]).compute() == 0.0
    assert ProfitFactor().from_trades([-2.0]).compute() == 0.0

def test_profit_factor_column_selection_and_input_method_contract() -> None:
    frame = pd.DataFrame({'strategy': [0.1, -0.05], 'other': [9.0, 8.0]})
    assert ProfitFactor().from_returns(frame, column='strategy').compute() == pytest.approx(2.0)
    with pytest.raises(ValueError):
        ProfitFactor().from_returns(frame)
    unbound = ProfitFactor()
    with pytest.raises(ValueError):
        unbound.append(0.01)
