from __future__ import annotations

import empyrical
import numpy as np
import pytest

from taflow.metrics.downside_deviation import DownsideDeviation


def period_rate(annual_rate: float, periods_per_year: float) -> float:
    return float(np.expm1(np.log1p(annual_rate) / periods_per_year))


@pytest.mark.parametrize(
    ("returns", "annual_required_return", "periods_per_year"),
    [
        (np.array([-0.01]), 0.0, 252.0),
        (np.array([0.10, -0.20, 0.05]), 0.0, 12.0),
        (np.zeros(32), 0.05, 365.0),
        (np.linspace(-0.04, 0.05, 101), -0.02, 52.0),
        (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 0.07, 252.0),
        (np.random.default_rng(20260811).normal(0.0004, 0.012, 513), 0.03, 252.0),
    ],
)
def test_downside_deviation_matches_empyrical(
    returns: np.ndarray,
    annual_required_return: float,
    periods_per_year: float,
) -> None:
    actual = DownsideDeviation.from_returns(
        returns,
        annual_required_return=annual_required_return,
        periods_per_year=periods_per_year,
    ).compute()
    expected = float(
        empyrical.downside_risk(
            returns,
            required_return=period_rate(annual_required_return, periods_per_year),
            annualization=periods_per_year,
        )
    )
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_downside_deviation_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    parameters = {"annual_required_return": 0.03, "periods_per_year": 12.0}
    expected = DownsideDeviation.from_returns(returns, **parameters).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    pnl = np.array([10.0, -22.0, 4.4])

    assert DownsideDeviation.from_equity(equity, **parameters).compute() == pytest.approx(expected)
    assert DownsideDeviation.from_pnl(
        pnl, initial_equity=100.0, **parameters
    ).compute() == pytest.approx(expected)
    assert DownsideDeviation.from_log_returns(
        np.log1p(returns), **parameters
    ).compute() == pytest.approx(expected)

    state = DownsideDeviation.from_returns([], **parameters)
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is not None
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_downside_deviation_missing_constant_and_invalid_values() -> None:
    assert DownsideDeviation.from_returns([]).compute() is None
    assert DownsideDeviation.from_returns([0.25]).compute() == 0.0
    assert len(DownsideDeviation.from_returns([np.nan, 0.01, 0.02])) == 2
    with pytest.raises(ValueError):
        DownsideDeviation.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        DownsideDeviation.from_returns([np.inf])
    with pytest.raises(ValueError):
        DownsideDeviation.from_returns([0.01], periods_per_year=0.0)
    with pytest.raises(ValueError):
        DownsideDeviation.from_returns([0.01], annual_required_return=-1.0)


def test_downside_deviation_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        DownsideDeviation()
