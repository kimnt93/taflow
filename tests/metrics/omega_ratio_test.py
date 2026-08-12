from __future__ import annotations
import empyrical
import numpy as np
import pytest
from taflow.metrics.omega_ratio import OmegaRatio

def _empyrical_omega(returns: np.ndarray, periods_per_year: float, annual_required_return: float) -> float | None:
    usable = returns[~np.isnan(returns)]
    expected = float(empyrical.omega_ratio(usable, required_return=annual_required_return, annualization=periods_per_year))
    return expected if np.isfinite(expected) else None

@pytest.mark.parametrize(('returns', 'periods_per_year', 'annual_required_return'), [(np.array([0.01, -0.01]), 252.0, 0.0), (np.array([0.1, -0.2, 0.05]), 12.0, 0.04), (np.linspace(-0.04, 0.05, 101), 52.0, -0.01), (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 252.0, 0.03), (np.random.default_rng(20260811).normal(0.0004, 0.012, 513), 365.0, 0.05)])
def test_omega_ratio_matches_empyrical(returns: np.ndarray, periods_per_year: float, annual_required_return: float) -> None:
    actual = OmegaRatio(periods_per_year=periods_per_year, annual_required_return=annual_required_return).from_returns(returns).compute()
    expected = _empyrical_omega(returns, periods_per_year, annual_required_return)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_omega_ratio_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05])
    settings = {'periods_per_year': 12.0, 'annual_required_return': 0.04}
    expected = OmegaRatio(**settings).from_returns(returns).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    pnl = np.array([10.0, -22.0, 4.4])
    assert OmegaRatio(**settings).from_equity(equity).compute() == pytest.approx(expected)
    assert OmegaRatio(**settings).from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert OmegaRatio(**settings).from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = OmegaRatio(**settings).from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_omega_ratio_missing_boundaries_and_invalid_values() -> None:
    assert OmegaRatio().from_returns([0.25]).compute() is None
    assert OmegaRatio().from_returns([0.25, 0.1]).compute() is None
    assert OmegaRatio().from_returns([-0.25, -0.1]).compute() == 0.0
    assert len(OmegaRatio().from_returns([np.nan, 0.01, -0.02])) == 2
    with pytest.raises(ValueError):
        OmegaRatio(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        OmegaRatio().from_returns([np.inf])
    with pytest.raises(ValueError):
        OmegaRatio(periods_per_year=0.0).from_returns([0.01, -0.02])
    with pytest.raises(ValueError):
        OmegaRatio(annual_required_return=-1.0).from_returns([0.01, -0.02])

def test_omega_ratio_requires_semantic_ingestion_before_append() -> None:
    with pytest.raises(ValueError):
        OmegaRatio().append(0.01)
