from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.expectancy import Expectancy

def _quantstats_component_expectancy(values: np.ndarray) -> float | None:
    """Combine pinned QuantStats components with the frozen probability denominator."""
    usable = values[~np.isnan(values)]
    if usable.size == 0:
        return None
    series = pd.Series(usable)
    wins = int(np.count_nonzero(usable > 0.0))
    losses = int(np.count_nonzero(usable < 0.0))
    average_win = float(qs.avg_win(series, aggregate=None, compounded=True, prepare_returns=False))
    average_loss = float(qs.avg_loss(series, aggregate=None, compounded=True, prepare_returns=False))
    win_component = 0.0 if wins == 0 else wins / usable.size * average_win
    loss_component = 0.0 if losses == 0 else losses / usable.size * average_loss
    return win_component + loss_component

@pytest.mark.parametrize('values', [np.array([100.0, -40.0, 0.0, 20.0, -10.0]), np.array([100.0]), np.array([-100.0]), np.zeros(32), np.linspace(-400.0, 500.0, 101), np.array([np.nan, 20.0, -30.0, np.nan, 0.0, 10.0]), np.random.default_rng(20260811).normal(4.0, 120.0, 513)])
def test_expectancy_matches_quantstats_component_oracle(values: np.ndarray) -> None:
    actual = Expectancy().from_pnl(values).compute()
    expected = _quantstats_component_expectancy(values)
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-13)

def test_expectancy_breakevens_use_all_observation_denominator() -> None:
    assert Expectancy().from_trades([100.0, -40.0]).compute() == pytest.approx(30.0)
    assert Expectancy().from_trades([100.0, -40.0, 0.0]).compute() == pytest.approx(20.0)

def test_expectancy_input_methods_and_lifecycle_are_invariant() -> None:
    values = np.array([100.0, -40.0, 0.0, 20.0, -10.0])
    expected = Expectancy().from_pnl(values).compute()
    assert Expectancy().from_trades(values).compute() == expected
    state = Expectancy().from_pnl([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value == 100.0
    assert state.extend(values[1:3]) is state
    assert state.extend(values[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(values)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == pytest.approx(expected)

def test_expectancy_missing_invalid_and_input_method_contract() -> None:
    state = Expectancy().from_pnl([np.nan, 20.0, -10.0])
    assert len(state) == 2
    assert state.compute() == pytest.approx(5.0)
    assert Expectancy().from_pnl([]).compute() is None
    assert Expectancy().from_pnl([0.0, -0.0]).compute() == 0.0
    with pytest.raises(ValueError):
        Expectancy(nan_policy='raise').from_pnl([np.nan])
    with pytest.raises(ValueError):
        Expectancy().from_trades([np.inf])
    with pytest.raises(TypeError):
        Expectancy().from_pnl([1.0], initial_equity=100.0)
    unbound = Expectancy()
    with pytest.raises(ValueError):
        unbound.append(0.01)

def test_expectancy_column_selection() -> None:
    frame = pd.DataFrame({'pnl': [100.0, -40.0, 0.0], 'other': [9.0, 8.0, 7.0]})
    assert Expectancy().from_pnl(frame, column='pnl').compute() == pytest.approx(20.0)
    with pytest.raises(ValueError):
        Expectancy().from_pnl(frame)
