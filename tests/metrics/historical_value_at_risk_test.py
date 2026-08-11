from __future__ import annotations

import empyrical
import numpy as np
import pytest

from taflow.metrics.historical_value_at_risk import HistoricalValueAtRisk


@pytest.mark.parametrize("cutoff", [0.01, 0.05, 0.25, 0.5, 0.95])
@pytest.mark.parametrize(
    "returns",
    [
        np.array([-0.03]),
        np.zeros(32),
        np.linspace(-0.10, 0.08, 101),
        np.array([-0.05, -0.05, 0.0, 0.02, 0.02, 0.07]),
        np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
    ],
)
def test_historical_value_at_risk_matches_empyrical(
    returns: np.ndarray, cutoff: float
) -> None:
    actual = HistoricalValueAtRisk.from_returns(returns, cutoff=cutoff).compute()
    expected = float(empyrical.value_at_risk(returns, cutoff=cutoff))
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-15)


def test_historical_value_at_risk_omits_nan_before_oracle_comparison() -> None:
    returns = np.array([np.nan, -0.08, 0.01, np.nan, -0.02, 0.03])
    usable = returns[~np.isnan(returns)]
    actual = HistoricalValueAtRisk.from_returns(returns, cutoff=0.2).compute()
    expected = float(empyrical.value_at_risk(usable, cutoff=0.2))
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-15)
    assert len(HistoricalValueAtRisk.from_returns(returns)) == len(usable)


def test_historical_value_at_risk_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    expected = HistoricalValueAtRisk.from_returns(returns, cutoff=0.4).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    pnl = np.array([10.0, -22.0, 4.4])

    assert HistoricalValueAtRisk.from_equity(
        equity, cutoff=0.4
    ).compute() == pytest.approx(expected)
    assert HistoricalValueAtRisk.from_pnl(
        pnl, initial_equity=100.0, cutoff=0.4
    ).compute() == pytest.approx(expected)
    assert HistoricalValueAtRisk.from_log_returns(
        np.log1p(returns), cutoff=0.4
    ).compute() == pytest.approx(expected)

    state = HistoricalValueAtRisk.from_returns([], cutoff=0.4)
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value == returns[0]
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert state.value == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.compute() is None
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_historical_value_at_risk_rejects_invalid_inputs() -> None:
    for cutoff in [0.0, 1.0, -0.1, 1.1, np.nan, np.inf]:
        with pytest.raises(ValueError):
            HistoricalValueAtRisk.from_returns([0.01], cutoff=cutoff)
    with pytest.raises(ValueError):
        HistoricalValueAtRisk.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        HistoricalValueAtRisk.from_returns([np.inf])
    with pytest.raises(ValueError):
        HistoricalValueAtRisk.from_returns([-1.01])


def test_historical_value_at_risk_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        HistoricalValueAtRisk()
