"""External correctness and lifecycle tests for KellyCriterion."""
from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.kelly_criterion import KellyCriterion

def quantstats_kelly_criterion(values: np.ndarray) -> float | None:
    """Evaluate QuantStats and normalize its undefined NaN sentinel."""
    filtered = values[~np.isnan(values)]
    expected = float(qs.kelly_criterion(pd.Series(filtered), prepare_returns=False))
    return expected if np.isfinite(expected) else None

@pytest.mark.parametrize('returns', [np.array([0.1, -0.2]), np.array([0.1, 0.0, -0.2, 0.3, -0.1, 0.0]), np.linspace(-0.04, 0.05, 101), np.array([0.02, 0.02, -0.01, -0.01, 0.0, 0.02]), np.array([np.nan, 0.02, -0.03, np.nan, 0.0, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_kelly_criterion_matches_quantstats(returns: np.ndarray) -> None:
    actual = KellyCriterion().from_returns(returns).compute()
    expected = quantstats_kelly_criterion(returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_kelly_criterion_freezes_decisive_breakeven_handling() -> None:
    base = np.array([0.1, -0.05])
    with_breakevens = np.array([0.1, 0.0, -0.05, -0.0, 0.0])
    expected = quantstats_kelly_criterion(with_breakevens)
    assert KellyCriterion().from_returns(base).compute() == pytest.approx(expected)
    state = KellyCriterion().from_returns(with_breakevens)
    assert state.compute() == pytest.approx(expected)
    assert len(state) == with_breakevens.size

def test_kelly_criterion_trade_input_method_preserves_absolute_payoff_ratio() -> None:
    trades = np.array([100.0, -20.0, 0.0, 50.0, -10.0])
    expected = quantstats_kelly_criterion(trades)
    assert expected is not None
    assert KellyCriterion().from_trades(trades).compute() == pytest.approx(expected)

def test_kelly_criterion_lifecycle_is_chunk_and_reset_invariant() -> None:
    values = np.array([0.1, np.nan, 0.0, -0.2, 0.3, -0.1])
    expected = KellyCriterion().from_returns(values).compute()
    state = KellyCriterion().from_returns([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value is None
    assert state.extend(values[1:4]) is state
    assert state.extend(values[4:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == 5
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == pytest.approx(expected)

def test_kelly_criterion_undefined_and_invalid_contract() -> None:
    for values in ([], [0.0, -0.0], [0.1, 0.2], [-0.1, -0.2]):
        assert KellyCriterion().from_returns(values).compute() is None
    with pytest.raises(ValueError):
        KellyCriterion(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        KellyCriterion().from_returns([np.inf])
    with pytest.raises(ValueError):
        KellyCriterion().from_returns([-1.01])
    assert not hasattr(KellyCriterion, 'from_pnl')

def test_kelly_criterion_requires_semantic_input_method() -> None:
    metric = KellyCriterion()
    with pytest.raises(ValueError):
        metric.append(0.01)
