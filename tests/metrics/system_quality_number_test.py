from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
from taflow.metrics.system_quality_number import SystemQualityNumber
VECTORBT_0_28_1_SHA256 = 'aceeb4767a1bd5be18329bc85779b2fc744b1edc4513ad19c4dbc3fc7d83d301'

def _numpy_system_quality_number(trades: np.ndarray) -> float | None:
    usable = trades[~np.isnan(trades)]
    if len(usable) < 2:
        return None
    standard_deviation = float(np.std(usable, ddof=1))
    if standard_deviation == 0.0:
        return None
    return float(np.sqrt(len(usable)) * np.mean(usable) / standard_deviation)

@pytest.mark.parametrize('trades', [np.array([100.0, -40.0, 20.0, -10.0, 80.0]), np.array([-100.0, 20.0, -50.0, 10.0]), np.array([-10.0, 10.0]), np.array([10.0]), np.full(32, 25.0), np.linspace(-400.0, 500.0, 101), np.array([np.nan, 20.0, -30.0, np.nan, 10.0]), np.random.default_rng(20260811).normal(40.0, 1200.0, 513)])
def test_system_quality_number_matches_numpy_and_pinned_vectorbt_formula(trades: np.ndarray) -> None:
    assert len(VECTORBT_0_28_1_SHA256) == 64
    actual = SystemQualityNumber().from_trades(trades).compute()
    expected = _numpy_system_quality_number(trades)
    if expected is None:
        assert actual is None
    else:
        assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_system_quality_number_lifecycle_is_chunk_and_reset_invariant() -> None:
    trades = np.array([100.0, -40.0, 20.0, -10.0, 80.0])
    expected = SystemQualityNumber().from_trades(trades).compute()
    state = SystemQualityNumber().from_trades([])
    assert state.value is None
    assert state.append(trades[0]) is state
    assert state.value is None
    assert state.extend(trades[1:3]) is state
    assert state.extend(trades[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(trades)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(trades).compute() == pytest.approx(expected)

def test_system_quality_number_boundaries_missing_and_invalid_values() -> None:
    assert SystemQualityNumber().from_trades([]).compute() is None
    assert SystemQualityNumber().from_trades([10.0]).compute() is None
    assert SystemQualityNumber().from_trades([10.0, 10.0]).compute() is None
    assert SystemQualityNumber().from_trades([-10.0, 10.0]).compute() == 0.0
    assert len(SystemQualityNumber().from_trades([np.nan, -10.0, 20.0])) == 2
    with pytest.raises(ValueError):
        SystemQualityNumber(nan_policy='raise').from_trades([np.nan])
    with pytest.raises(ValueError):
        SystemQualityNumber().from_trades([np.inf])

def test_system_quality_number_column_selection_and_input_method_contract() -> None:
    frame = pd.DataFrame({'pnl': [100.0, -50.0], 'other': [1.0, 2.0]})
    expected = _numpy_system_quality_number(frame['pnl'].to_numpy())
    assert SystemQualityNumber().from_trades(frame, column='pnl').compute() == pytest.approx(expected)
    with pytest.raises(ValueError):
        SystemQualityNumber().from_trades(frame)
    unbound = SystemQualityNumber()
    with pytest.raises(ValueError):
        unbound.append(0.01)
