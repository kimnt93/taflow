from __future__ import annotations

import empyrical
import numpy as np
import pytest

from taflow.metrics.capture_ratio import CaptureRatio


def _empyrical_capture(
    returns: np.ndarray,
    benchmark_returns: np.ndarray,
    periods_per_year: float = 252.0,
) -> float | None:
    usable = ~(np.isnan(returns) | np.isnan(benchmark_returns))
    primary = returns[usable]
    benchmark = benchmark_returns[usable]
    if primary.size == 0:
        return None
    if periods_per_year == 252.0:
        expected = float(empyrical.capture(primary, benchmark, period="daily"))
        return expected if np.isfinite(expected) else None
    primary_cagr = float(empyrical.annual_return(primary, annualization=periods_per_year))
    benchmark_cagr = float(empyrical.annual_return(benchmark, annualization=periods_per_year))
    if benchmark_cagr == 0.0:
        return None
    return primary_cagr / benchmark_cagr


@pytest.mark.parametrize(
    ("returns", "benchmark_returns"),
    [
        (np.array([0.01]), np.array([0.02])),
        (
            np.array([0.10, -0.20, 0.05, 0.01]),
            np.array([0.02, -0.10, 0.01, 0.03]),
        ),
        (
            np.array([np.nan, 0.02, -0.03, np.nan, 0.01]),
            np.array([0.01, np.nan, -0.01, 0.02, 0.005]),
        ),
        (
            np.linspace(-0.004, 0.005, 101),
            np.linspace(-0.002, 0.004, 101),
        ),
        (
            np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
            np.random.default_rng(20260812).normal(0.0003, 0.009, 513),
        ),
    ],
)
def test_capture_ratio_matches_empyrical(
    returns: np.ndarray, benchmark_returns: np.ndarray
) -> None:
    actual = CaptureRatio.from_returns(returns, benchmark_returns).compute()
    expected = _empyrical_capture(returns, benchmark_returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-11, abs=1e-13)


def test_capture_ratio_uses_explicit_annualization() -> None:
    returns = np.array([0.02, -0.01, 0.03, 0.005])
    benchmark = np.array([0.01, -0.005, 0.015, 0.002])
    actual = CaptureRatio.from_returns(
        returns, benchmark, periods_per_year=12.0
    ).compute()
    expected = _empyrical_capture(returns, benchmark, 12.0)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_capture_ratio_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    benchmark_returns = np.array([0.02, -0.10, 0.01])
    expected = CaptureRatio.from_returns(
        returns, benchmark_returns, periods_per_year=12.0
    ).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    benchmark_equity = np.array([200.0, 204.0, 183.6, 185.436])

    assert CaptureRatio.from_equity(
        equity, benchmark_equity, periods_per_year=12.0
    ).compute() == pytest.approx(expected)
    assert CaptureRatio.from_pnl(
        np.array([10.0, -22.0, 4.4]),
        np.array([4.0, -20.4, 1.836]),
        initial_equity=100.0,
        benchmark_initial_equity=200.0,
        periods_per_year=12.0,
    ).compute() == pytest.approx(expected)
    assert CaptureRatio.from_log_returns(
        np.log1p(returns),
        np.log1p(benchmark_returns),
        periods_per_year=12.0,
    ).compute() == pytest.approx(expected)

    state = CaptureRatio.from_returns([], [], periods_per_year=12.0)
    assert state.value is None
    assert state.append(returns[0], benchmark_returns[0]) is state
    assert state.extend(returns[1:], benchmark_returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns, benchmark_returns).compute() == pytest.approx(expected)


def test_capture_ratio_pairwise_missing_minimum_and_zero_benchmark() -> None:
    returns = np.array([np.nan, 0.01, 0.05, -0.02, np.nan])
    benchmark = np.array([0.02, 0.00, np.nan, -0.01, np.nan])
    actual = CaptureRatio.from_returns(returns, benchmark)
    assert len(actual) == 2
    assert actual.compute() == pytest.approx(_empyrical_capture(returns, benchmark))
    assert CaptureRatio.from_returns([0.25], [0.10]).compute() is not None
    assert CaptureRatio.from_returns([0.01, 0.02], [0.0, 0.0]).compute() is None


def test_capture_ratio_rejects_misalignment_without_mutation() -> None:
    state = CaptureRatio.from_returns([0.01, 0.02], [0.03, 0.01])
    original_value = state.value
    original_length = len(state)
    with pytest.raises(ValueError, match="equal length"):
        state.extend([0.03, 0.04], [0.02])
    assert len(state) == original_length
    assert state.value == original_value


def test_capture_ratio_missing_invalid_and_factory_contracts() -> None:
    with pytest.raises(ValueError):
        CaptureRatio.from_returns([np.nan], [0.0], nan_policy="raise")
    with pytest.raises(ValueError):
        CaptureRatio.from_returns([np.inf], [0.0])
    with pytest.raises(ValueError):
        CaptureRatio.from_returns([0.01, 0.02], [0.01])
    with pytest.raises(ValueError):
        CaptureRatio.from_returns([0.01], [0.02], periods_per_year=0.0)
    with pytest.raises(TypeError):
        CaptureRatio()
