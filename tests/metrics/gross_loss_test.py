from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.gross_loss import GrossLoss

def _numpy_gross_loss(values: np.ndarray) -> float | None:
    usable = values[~np.isnan(values)]
    if len(usable) == 0:
        return None
    return float(usable[usable < 0.0].sum())

@pytest.mark.parametrize('pnl', [np.array([100.0, -40.0, 0.0, 20.0, -10.0]), np.array([-200.0]), np.array([200.0]), np.zeros(32), np.linspace(-400.0, 500.0, 101), np.array([np.nan, 20.0, -30.0, np.nan, 10.0]), np.random.default_rng(20260811).normal(40.0, 1200.0, 513)])
def test_gross_loss_matches_numpy(pnl: np.ndarray) -> None:
    actual = GrossLoss().from_pnl(pnl).compute()
    expected = _numpy_gross_loss(pnl)
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-12)

def test_gross_loss_matches_quantstats_profit_factor_denominator() -> None:
    pnl = np.array([0.1, -0.04, 0.0, 0.02, -0.01])
    gross_loss = GrossLoss().from_pnl(pnl).compute()
    gross_profit = float(pnl[pnl > 0.0].sum())
    profit_factor = float(qs.profit_factor(pd.Series(pnl), prepare_returns=False))
    assert gross_loss is not None
    assert abs(gross_loss) == pytest.approx(gross_profit / profit_factor)

def test_gross_loss_pnl_trade_input_methods_and_lifecycle_are_invariant() -> None:
    values = np.array([100.0, -40.0, 0.0, 20.0, -10.0])
    expected = GrossLoss().from_pnl(values).compute()
    assert GrossLoss().from_trades(values).compute() == expected
    state = GrossLoss().from_pnl([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value == 0.0
    assert state.extend(values[1:3]) is state
    assert state.extend(values[3:]) is state
    assert state.compute() == expected
    assert len(state) == len(values)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == expected

def test_gross_loss_boundaries_missing_invalid_and_input_method_contract() -> None:
    assert GrossLoss().from_pnl([]).compute() is None
    assert GrossLoss().from_pnl([0.0, 100.0]).compute() == 0.0
    assert GrossLoss().from_pnl([-1000.0, 50.0]).compute() == -1000.0
    assert len(GrossLoss().from_trades([np.nan, -10.0, 20.0])) == 2
    with pytest.raises(ValueError):
        GrossLoss(nan_policy='raise').from_pnl([np.nan])
    with pytest.raises(ValueError):
        GrossLoss().from_trades([np.inf])
    unbound = GrossLoss()
    with pytest.raises(ValueError):
        unbound.append(0.01)
