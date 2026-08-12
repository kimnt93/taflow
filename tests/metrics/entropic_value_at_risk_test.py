from __future__ import annotations
import numpy as np
import pytest
from scipy import optimize, special
from taflow.metrics.entropic_value_at_risk import EntropicValueAtRisk
RISKFOLIO_COMMIT = '632a9e48fbaf2b9f8e83864a492332364b6ed32c'
RISK_FUNCTIONS_SHA256 = 'eed72dceb7024c9ead811fa12d8e834618604f8db9c4fbb6367ce4d2a3992719'

def _riskfolio_empirical_evar(returns: np.ndarray, cutoff: float) -> float:
    """Independently optimize Riskfolio-Lib's published EVaR_Hist formula."""
    losses = -returns[~np.isnan(returns)]
    if len(losses) == 0:
        raise ValueError('oracle requires one observation')
    maximum_loss = float(np.max(losses))
    if np.ptp(losses) == 0.0:
        return maximum_loss

    def objective(log_z: float) -> float:
        z = np.exp(log_z)
        return float(z * (special.logsumexp(losses / z) - np.log(len(losses) * cutoff)))
    scale = max(float(np.ptp(losses)), np.finfo(np.float64).tiny)
    result = optimize.minimize_scalar(objective, bounds=(np.log(scale) - 40.0, np.log(scale) + 40.0), method='bounded', options={'xatol': 1e-13, 'maxiter': 1000})
    assert result.success
    return min(float(result.fun), maximum_loss)

@pytest.mark.parametrize(('returns', 'cutoff'), [(np.linspace(-0.04, 0.05, 101), 0.05), (np.sin(np.arange(257) * 0.37) * 0.03, 0.1), (np.random.default_rng(20260812).normal(0.0004, 0.012, 513), 0.025), (np.r_[np.nan, np.linspace(-0.03, 0.025, 128), np.nan], 0.2), (np.repeat(0.0125, 64), 0.05), (np.array([0.02, -0.01, -0.04, 0.03]), 0.05)])
def test_entropic_value_at_risk_matches_riskfolio_formula(returns: np.ndarray, cutoff: float) -> None:
    assert len(RISKFOLIO_COMMIT) == 40 and len(RISK_FUNCTIONS_SHA256) == 64
    expected = _riskfolio_empirical_evar(returns, cutoff)
    actual = EntropicValueAtRisk(cutoff=cutoff).from_returns(returns).compute()
    assert actual == pytest.approx(expected, rel=2e-10, abs=2e-12)

def test_entropic_value_at_risk_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.sin(np.arange(64) * 0.71) * 0.025
    expected = EntropicValueAtRisk(cutoff=0.1).from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert EntropicValueAtRisk(cutoff=0.1).from_equity(equity).compute() == pytest.approx(expected)
    assert EntropicValueAtRisk(cutoff=0.1).from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert EntropicValueAtRisk(cutoff=0.1).from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = EntropicValueAtRisk(cutoff=0.1).from_returns([])
    assert state.value is None
    assert state.extend(returns[:32]) is state
    first = state.compute()
    assert state.compute() == first
    assert state.append(float(returns[32])) is state
    assert state.extend(returns[33:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state and len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_entropic_value_at_risk_edges_and_validation() -> None:
    assert EntropicValueAtRisk().from_returns([]).compute() is None
    assert EntropicValueAtRisk().from_returns([0.0125]).compute() == pytest.approx(-0.0125)
    assert EntropicValueAtRisk().from_returns([0.0125] * 64).compute() == pytest.approx(-0.0125)
    assert EntropicValueAtRisk(cutoff=0.05).from_returns([0.02, -0.04]).compute() == pytest.approx(0.04)
    assert len(EntropicValueAtRisk().from_returns([np.nan, 0.02, -0.01])) == 2
    with pytest.raises(ValueError):
        EntropicValueAtRisk(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        EntropicValueAtRisk().from_returns([np.inf])
    with pytest.raises(ValueError):
        EntropicValueAtRisk().from_returns([-1.01])
    for cutoff in [0.0, 1.0, np.nan, np.inf]:
        with pytest.raises(ValueError):
            EntropicValueAtRisk(cutoff=cutoff).from_returns([0.01])

def test_entropic_value_at_risk_requires_semantic_input_method() -> None:
    metric = EntropicValueAtRisk()
    with pytest.raises(ValueError):
        metric.append(0.01)
