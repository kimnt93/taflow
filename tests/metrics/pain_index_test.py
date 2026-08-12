from __future__ import annotations
import numpy as np
import pytest
from taflow.metrics.pain_index import PainIndex
PERFORMANCE_ANALYTICS_2_1_0_SHA256 = 'fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d'

def _performance_analytics_source_convention(returns: np.ndarray) -> float | None:
    """Translate pinned PainIndex/DrawdownPeak source for specification parity."""
    usable = returns[~np.isnan(returns)]
    if len(usable) == 0:
        return None
    wealth = np.cumprod(1.0 + usable)
    peaks = np.maximum.accumulate(np.r_[1.0, wealth])[1:]
    return float(np.mean(np.abs(wealth / peaks - 1.0)))

@pytest.mark.parametrize('returns', [np.array([0.02, -0.01, 0.015, -0.03, 0.01]), np.array([-0.2]), np.array([0.2]), np.zeros(32), np.linspace(-0.04, 0.05, 101), np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_pain_index_matches_pinned_performanceanalytics_source_convention(returns: np.ndarray) -> None:
    assert len(PERFORMANCE_ANALYTICS_2_1_0_SHA256) == 64
    actual = PainIndex().from_returns(returns).compute()
    expected = _performance_analytics_source_convention(returns)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_pain_index_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05, -0.25, 0.1])
    expected = PainIndex().from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert PainIndex().from_equity(equity).compute() == pytest.approx(expected)
    assert PainIndex().from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert PainIndex().from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = PainIndex().from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.extend(returns[1:3]) is state
    assert state.extend(returns[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_pain_index_uses_phantom_wealth_but_excludes_it_from_divisor() -> None:
    assert PainIndex().from_returns([-0.2]).compute() == pytest.approx(0.2)
    assert PainIndex().from_returns([0.2]).compute() == pytest.approx(0.0)
    assert PainIndex().from_returns([0.1, -0.2, 0.25]).compute() == pytest.approx(0.2 / 3.0)

def test_pain_index_missing_and_invalid_values() -> None:
    filtered = np.array([0.1, -0.2, 0.05])
    state = PainIndex().from_returns([0.1, np.nan, -0.2, 0.05])
    assert len(state) == len(filtered)
    assert state.compute() == pytest.approx(_performance_analytics_source_convention(filtered))
    with pytest.raises(ValueError):
        PainIndex(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        PainIndex().from_returns([np.inf])
    with pytest.raises(ValueError):
        PainIndex().from_returns([-1.01])

def test_pain_index_requires_semantic_ingestion_before_append() -> None:
    with pytest.raises(ValueError):
        PainIndex().append(0.01)
