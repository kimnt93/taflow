from __future__ import annotations

import numpy as np
import pytest

from taflow.metrics.tracking_error import TrackingError


def expected_tracking_error(
    returns: np.ndarray,
    benchmark_returns: np.ndarray,
    periods_per_year: float,
    annualized: bool,
) -> float:
    """Independent NumPy oracle with explicit pairwise missing omission."""
    usable = ~(np.isnan(returns) | np.isnan(benchmark_returns))
    scale = np.sqrt(periods_per_year) if annualized else 1.0
    return float(np.std(returns[usable] - benchmark_returns[usable], ddof=1) * scale)


@pytest.mark.parametrize("annualized", [False, True])
@pytest.mark.parametrize("periods_per_year", [1.0, 12.0, 252.0, 8760.0])
def test_tracking_error_matches_numpy(
    annualized: bool, periods_per_year: float
) -> None:
    returns = np.array([0.01, np.nan, -0.03, 0.02, 0.005, -0.01])
    benchmark = np.array([0.005, 0.01, -0.01, np.nan, 0.002, -0.02])
    actual = TrackingError.from_returns(
        returns,
        benchmark,
        periods_per_year=periods_per_year,
        annualized=annualized,
    ).compute()
    expected = expected_tracking_error(
        returns, benchmark, periods_per_year, annualized
    )
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-15)


def test_tracking_error_random_constant_and_minimum_cases() -> None:
    random = np.random.default_rng(20260811)
    benchmark = random.normal(0.0003, 0.01, 513)
    returns = benchmark + random.normal(0.0001, 0.004, 513)
    expected = expected_tracking_error(returns, benchmark, 252.0, True)
    assert TrackingError.from_returns(returns, benchmark).compute() == pytest.approx(
        expected, rel=1e-13, abs=1e-15
    )
    assert TrackingError.from_returns([0.01], [0.00]).compute() is None
    assert TrackingError.from_returns([0.02, 0.02], [0.01, 0.01]).compute() == 0.0


def test_tracking_error_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    benchmark_returns = np.array([0.02, -0.10, 0.01])
    expected = TrackingError.from_returns(
        returns, benchmark_returns, periods_per_year=12.0
    ).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    benchmark_equity = np.array([200.0, 204.0, 183.6, 185.436])

    assert TrackingError.from_equity(
        equity, benchmark_equity, periods_per_year=12.0
    ).compute() == pytest.approx(expected)
    assert TrackingError.from_pnl(
        np.array([10.0, -22.0, 4.4]),
        np.array([4.0, -20.4, 1.836]),
        initial_equity=100.0,
        benchmark_initial_equity=200.0,
        periods_per_year=12.0,
    ).compute() == pytest.approx(expected)
    assert TrackingError.from_log_returns(
        np.log1p(returns),
        np.log1p(benchmark_returns),
        periods_per_year=12.0,
    ).compute() == pytest.approx(expected)

    state = TrackingError.from_returns([], [], periods_per_year=12.0)
    assert state.value is None
    assert state.append(returns[0], benchmark_returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:], benchmark_returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns, benchmark_returns).compute() == pytest.approx(expected)


def test_tracking_error_rejects_misalignment_without_mutation() -> None:
    state = TrackingError.from_returns([0.01, 0.02], [0.00, 0.01])
    original_value = state.value
    original_length = len(state)
    with pytest.raises(ValueError, match="equal length"):
        state.extend([0.03, 0.04], [0.02])
    assert len(state) == original_length
    assert state.value == original_value


def test_tracking_error_missing_and_invalid_values() -> None:
    state = TrackingError.from_returns(
        [np.nan, 0.01, 0.02], [0.00, 0.00, np.nan]
    )
    assert len(state) == 1
    assert state.compute() is None
    with pytest.raises(ValueError):
        TrackingError.from_returns([np.nan], [0.0], nan_policy="raise")
    with pytest.raises(ValueError):
        TrackingError.from_returns([np.inf], [0.0])
    with pytest.raises(ValueError):
        TrackingError.from_returns([0.01, 0.02], [0.01])
    with pytest.raises(ValueError):
        TrackingError.from_returns([0.01, 0.02], [0.00, 0.01], periods_per_year=0.0)


def test_tracking_error_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        TrackingError()
