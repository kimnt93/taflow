from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.average_win import AverageWin

def _quantstats_average_win(values: np.ndarray) -> float | None:
    expected = float(qs.avg_win(pd.Series(values), aggregate=None, compounded=True, prepare_returns=False))
    return expected if np.isfinite(expected) else None

@pytest.mark.parametrize('returns', [np.array([0.02, -0.01, 0.0, 0.03, -0.025, 0.01]), np.array([0.1]), np.zeros(32), np.linspace(-0.04, 0.05, 101), np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_average_win_matches_quantstats(returns: np.ndarray) -> None:
    actual = AverageWin().from_returns(returns).compute()
    expected = _quantstats_average_win(returns)
    if expected is None:
        assert actual is None
    else:
        assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_average_win_raw_pnl_and_trade_input_methods_match_quantstats() -> None:
    observations = np.array([100.0, -40.0, 0.0, 20.0, -10.0])
    expected = _quantstats_average_win(observations)
    assert expected == 60.0
    assert AverageWin().from_pnl(observations).compute() == expected
    assert AverageWin().from_trades(observations).compute() == expected

def test_average_win_lifecycle_is_chunk_and_reset_invariant() -> None:
    values = np.array([0.02, -0.01, 0.0, 0.03, -0.025, 0.01])
    expected = AverageWin().from_returns(values).compute()
    state = AverageWin().from_returns([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value == values[0]
    assert state.extend(values[1:3]) is state
    assert state.extend(values[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(values)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == pytest.approx(expected)

def test_average_win_boundaries_missing_and_invalid_values() -> None:
    assert AverageWin().from_returns([]).compute() is None
    assert AverageWin().from_returns([-0.25, 0.0]).compute() is None
    assert AverageWin().from_returns([0.25]).compute() == 0.25
    assert len(AverageWin().from_returns([np.nan, 0.01, -0.02])) == 2
    with pytest.raises(ValueError):
        AverageWin(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        AverageWin().from_returns([np.inf])
    with pytest.raises(ValueError):
        AverageWin().from_returns([-1.01])
    assert AverageWin().from_pnl([-1000.0, 50.0]).compute() == 50.0
    assert AverageWin().from_trades([-1000.0, 50.0]).compute() == 50.0

def test_average_win_requires_semantic_input_method() -> None:
    metric = AverageWin()
    with pytest.raises(ValueError):
        metric.append(0.01)
