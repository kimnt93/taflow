from __future__ import annotations
import empyrical
import numpy as np
import pytest
from taflow.metrics.annualized_volatility import AnnualizedVolatility

@pytest.mark.parametrize(('returns', 'periods_per_year'), [(np.array([0.01, -0.01]), 252.0), (np.array([0.1, -0.2, 0.05]), 12.0), (np.zeros(32), 365.0), (np.linspace(-0.04, 0.05, 101), 52.0), (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 252.0), (np.random.default_rng(20260811).normal(0.0004, 0.012, 513), 252.0)])
def test_annualized_volatility_matches_empyrical(returns: np.ndarray, periods_per_year: float) -> None:
    actual = AnnualizedVolatility(periods_per_year=periods_per_year).from_returns(returns).compute()
    expected = float(empyrical.annual_volatility(returns, annualization=periods_per_year))
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_annualized_volatility_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05])
    expected = AnnualizedVolatility(periods_per_year=12.0).from_returns(returns).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    pnl = np.array([10.0, -22.0, 4.4])
    assert AnnualizedVolatility(periods_per_year=12.0).from_equity(equity).compute() == pytest.approx(expected)
    assert AnnualizedVolatility(periods_per_year=12.0).from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert AnnualizedVolatility(periods_per_year=12.0).from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = AnnualizedVolatility(periods_per_year=12.0).from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_annualized_volatility_missing_constant_and_invalid_values() -> None:
    assert AnnualizedVolatility().from_returns([0.25]).compute() is None
    assert AnnualizedVolatility().from_returns([0.25, 0.25]).compute() == 0.0
    assert len(AnnualizedVolatility().from_returns([np.nan, 0.01, 0.02])) == 2
    with pytest.raises(ValueError):
        AnnualizedVolatility(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        AnnualizedVolatility().from_returns([np.inf])
    with pytest.raises(ValueError):
        AnnualizedVolatility(periods_per_year=0.0).from_returns([0.01, 0.02])

def test_annualized_volatility_requires_semantic_ingestion_before_append() -> None:
    with pytest.raises(ValueError):
        AnnualizedVolatility().append(0.01)
