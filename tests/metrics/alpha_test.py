from __future__ import annotations

import empyrical
import numpy as np
import pytest

from taflow.metrics.alpha import Alpha


def _pairwise_usable(
    returns: np.ndarray, benchmark_returns: np.ndarray
) -> tuple[np.ndarray, np.ndarray]:
    usable = ~(np.isnan(returns) | np.isnan(benchmark_returns))
    return returns[usable], benchmark_returns[usable]


def _empyrical_alpha(
    returns: np.ndarray,
    benchmark_returns: np.ndarray,
    periods_per_year: float,
    annual_risk_free_rate: float,
) -> float | None:
    primary, benchmark = _pairwise_usable(returns, benchmark_returns)
    period_risk_free_rate = np.expm1(
        np.log1p(annual_risk_free_rate) / periods_per_year
    )
    expected = float(
        empyrical.alpha_aligned(
            primary,
            benchmark,
            risk_free=period_risk_free_rate,
            annualization=periods_per_year,
        )
    )
    return expected if np.isfinite(expected) else None


@pytest.mark.parametrize(
    ("returns", "benchmark_returns", "periods_per_year", "annual_risk_free_rate"),
    [
        (np.array([0.01, -0.01]), np.array([0.02, -0.02]), 252.0, 0.0),
        (
            np.array([0.10, -0.20, 0.05, 0.01]),
            np.array([0.02, -0.10, 0.01, 0.03]),
            12.0,
            0.04,
        ),
        (
            np.array([np.nan, 0.02, -0.03, np.nan, 0.01]),
            np.array([0.01, np.nan, -0.01, 0.02, 0.005]),
            52.0,
            -0.01,
        ),
        (
            np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
            np.random.default_rng(20260812).normal(0.0003, 0.009, 513),
            365.0,
            0.05,
        ),
    ],
)
def test_alpha_matches_empyrical(
    returns: np.ndarray,
    benchmark_returns: np.ndarray,
    periods_per_year: float,
    annual_risk_free_rate: float,
) -> None:
    actual = Alpha.from_returns(
        returns,
        benchmark_returns,
        periods_per_year=periods_per_year,
        annual_risk_free_rate=annual_risk_free_rate,
    ).compute()
    expected = _empyrical_alpha(
        returns,
        benchmark_returns,
        periods_per_year,
        annual_risk_free_rate,
    )
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_alpha_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    benchmark_returns = np.array([0.02, -0.10, 0.01])
    settings = {"periods_per_year": 12.0, "annual_risk_free_rate": 0.04}
    expected = Alpha.from_returns(returns, benchmark_returns, **settings).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    benchmark_equity = np.array([200.0, 204.0, 183.6, 185.436])

    assert Alpha.from_equity(
        equity, benchmark_equity, **settings
    ).compute() == pytest.approx(expected)
    assert Alpha.from_pnl(
        np.array([10.0, -22.0, 4.4]),
        np.array([4.0, -20.4, 1.836]),
        initial_equity=100.0,
        benchmark_initial_equity=200.0,
        **settings,
    ).compute() == pytest.approx(expected)
    assert Alpha.from_log_returns(
        np.log1p(returns), np.log1p(benchmark_returns), **settings
    ).compute() == pytest.approx(expected)

    state = Alpha.from_returns([], [], **settings)
    assert state.value is None
    assert state.append(returns[0], benchmark_returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:], benchmark_returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns, benchmark_returns).compute() == pytest.approx(expected)


def test_alpha_pairwise_missing_minimum_and_zero_variance() -> None:
    returns = np.array([np.nan, 0.01, 0.05, -0.02, np.nan])
    benchmark = np.array([0.02, 0.00, np.nan, -0.01, np.nan])
    state = Alpha.from_returns(returns, benchmark, periods_per_year=1.0)
    assert len(state) == 2
    assert state.compute() == pytest.approx(
        _empyrical_alpha(returns, benchmark, 1.0, 0.0)
    )
    assert Alpha.from_returns([0.25], [0.10]).compute() is None
    assert Alpha.from_returns([0.01, 0.02], [0.10, 0.10]).compute() is None


def test_alpha_rejects_misalignment_without_mutation() -> None:
    state = Alpha.from_returns([0.01, 0.02], [0.00, 0.01])
    original_value = state.value
    original_length = len(state)
    with pytest.raises(ValueError, match="equal length"):
        state.extend([0.03, 0.04], [0.02])
    assert len(state) == original_length
    assert state.value == original_value


def test_alpha_rejects_invalid_values_configuration_and_constructor() -> None:
    with pytest.raises(ValueError):
        Alpha.from_returns([np.nan], [0.0], nan_policy="raise")
    with pytest.raises(ValueError):
        Alpha.from_returns([np.inf], [0.0])
    with pytest.raises(ValueError):
        Alpha.from_returns([0.01, 0.02], [0.01])
    with pytest.raises(ValueError):
        Alpha.from_returns([0.01, 0.02], [0.00, 0.01], periods_per_year=0.0)
    with pytest.raises(ValueError):
        Alpha.from_returns(
            [0.01, 0.02], [0.00, 0.01], annual_risk_free_rate=-1.0
        )
    with pytest.raises(TypeError):
        Alpha()
