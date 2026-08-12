from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.payoff_ratio import PayoffRatio

def _quantstats_payoff_ratio(values: np.ndarray) -> float | None:
    expected = float(qs.payoff_ratio(pd.Series(values), prepare_returns=False))
    return expected if np.isfinite(expected) else None

@pytest.mark.parametrize('returns', [np.array([0.02, -0.01, 0.0, 0.03, -0.025, 0.01]), np.array([0.1, -0.1]), np.zeros(32), np.linspace(-0.04, 0.05, 101), np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_payoff_ratio_matches_quantstats(returns: np.ndarray) -> None:
    actual = PayoffRatio().from_returns(returns).compute()
    expected = _quantstats_payoff_ratio(returns)
    if expected is None:
        assert actual is None
    else:
        assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_payoff_ratio_raw_pnl_and_trade_input_methods_preserve_values() -> None:
    observations = np.array([100.0, -40.0, 0.0, 20.0, -10.0])
    expected = 60.0 / 25.0
    assert PayoffRatio().from_pnl(observations).compute() == pytest.approx(expected)
    assert PayoffRatio().from_trades(observations).compute() == pytest.approx(expected)

def test_payoff_ratio_lifecycle_is_chunk_and_reset_invariant() -> None:
    values = np.array([0.02, -0.01, 0.0, 0.03, -0.025, 0.01])
    expected = PayoffRatio().from_returns(values).compute()
    state = PayoffRatio().from_returns([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value is None
    assert state.extend(values[1:3]) is state
    assert state.extend(values[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(values)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == pytest.approx(expected)

def test_payoff_ratio_boundaries_missing_and_invalid_values() -> None:
    assert PayoffRatio().from_returns([]).compute() is None
    assert PayoffRatio().from_returns([0.25, 0.0]).compute() is None
    assert PayoffRatio().from_returns([-0.25, 0.0]).compute() is None
    assert PayoffRatio().from_returns([0.25, -0.5]).compute() == 0.5
    assert len(PayoffRatio().from_returns([np.nan, 0.01, -0.02])) == 2
    with pytest.raises(ValueError):
        PayoffRatio(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        PayoffRatio().from_returns([np.inf])
    with pytest.raises(ValueError):
        PayoffRatio().from_returns([-1.01])
    assert PayoffRatio().from_pnl([-1000.0, 50.0]).compute() == 0.05
    assert PayoffRatio().from_trades([-1000.0, 50.0]).compute() == 0.05

def test_payoff_ratio_requires_semantic_input_method() -> None:
    metric = PayoffRatio()
    with pytest.raises(ValueError):
        metric.append(0.01)
