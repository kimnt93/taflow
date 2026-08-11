from __future__ import annotations

import empyrical
import numpy as np
import pytest

from taflow.metrics.up_down_capture_ratio import UpDownCaptureRatio


def _empyrical_up_down_capture(
    returns: np.ndarray,
    benchmark_returns: np.ndarray,
    periods_per_year: float = 252.0,
) -> float | None:
    pairwise = ~(np.isnan(returns) | np.isnan(benchmark_returns))
    primary = returns[pairwise]
    benchmark = benchmark_returns[pairwise]
    up = benchmark > 0.0
    down = benchmark < 0.0
    if not np.any(up) or not np.any(down):
        return None
    if periods_per_year == 252.0:
        result = float(
            empyrical.up_down_capture(primary, benchmark, period="daily")
        )
    else:
        up_capture = float(
            empyrical.annual_return(primary[up], annualization=periods_per_year)
            / empyrical.annual_return(
                benchmark[up], annualization=periods_per_year
            )
        )
        down_capture = float(
            empyrical.annual_return(primary[down], annualization=periods_per_year)
            / empyrical.annual_return(
                benchmark[down], annualization=periods_per_year
            )
        )
        result = up_capture / down_capture
    return result if np.isfinite(result) else None


@pytest.mark.parametrize(
    ("returns", "benchmark_returns"),
    [
        (np.array([0.01, -0.01]), np.array([0.02, -0.02])),
        (
            np.array([0.10, -0.20, 0.05, 0.01, -0.03]),
            np.array([0.02, -0.10, 0.0, 0.03, -0.02]),
        ),
        (
            np.array([np.nan, 0.02, -0.03, np.nan, 0.01, 0.005]),
            np.array([0.01, np.nan, 0.015, 0.02, -0.005, -0.002]),
        ),
        (
            np.linspace(-0.004, 0.005, 101),
            np.tile(np.array([-0.002, 0.0, 0.003, -0.003]), 26)[:101],
        ),
        (
            np.random.default_rng(20260829).normal(0.0004, 0.012, 513),
            np.random.default_rng(20260830).normal(0.0003, 0.009, 513),
        ),
    ],
)
def test_up_down_capture_ratio_matches_empyrical(
    returns: np.ndarray, benchmark_returns: np.ndarray
) -> None:
    actual = UpDownCaptureRatio.from_returns(returns, benchmark_returns).compute()
    expected = _empyrical_up_down_capture(returns, benchmark_returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-11, abs=1e-13)


def test_up_down_capture_ratio_uses_explicit_annualization() -> None:
    returns = np.array([0.02, -0.01, 0.03, -0.02])
    benchmark = np.array([0.01, -0.005, 0.015, -0.01])
    actual = UpDownCaptureRatio.from_returns(
        returns, benchmark, periods_per_year=12.0
    ).compute()
    expected = _empyrical_up_down_capture(returns, benchmark, 12.0)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    benchmark_returns = np.array([0.02, -0.10, 0.01])
    expected = UpDownCaptureRatio.from_returns(
        returns, benchmark_returns, periods_per_year=12.0
    ).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    benchmark_equity = np.array([200.0, 204.0, 183.6, 185.436])

    assert UpDownCaptureRatio.from_equity(
        equity, benchmark_equity, periods_per_year=12.0
    ).compute() == pytest.approx(expected)
    assert UpDownCaptureRatio.from_pnl(
        np.array([10.0, -22.0, 4.4]),
        np.array([4.0, -20.4, 1.836]),
        initial_equity=100.0,
        benchmark_initial_equity=200.0,
        periods_per_year=12.0,
    ).compute() == pytest.approx(expected)
    assert UpDownCaptureRatio.from_log_returns(
        np.log1p(returns),
        np.log1p(benchmark_returns),
        periods_per_year=12.0,
    ).compute() == pytest.approx(expected)

    state = UpDownCaptureRatio.from_returns([], [], periods_per_year=12.0)
    assert state.value is None
    assert state.append(returns[0], benchmark_returns[0]) is state
    assert state.compute() is None
    assert state.extend(returns[1:], benchmark_returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == 3
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns, benchmark_returns).compute() == pytest.approx(expected)


def test_pairwise_missing_filter_and_undefined_edges() -> None:
    returns = np.array([np.nan, 0.01, -0.05, -0.02, np.nan, 0.03])
    benchmark = np.array([0.02, 0.00, np.nan, -0.01, np.nan, 0.01])
    actual = UpDownCaptureRatio.from_returns(returns, benchmark)
    assert len(actual) == 3
    assert actual.compute() == pytest.approx(
        _empyrical_up_down_capture(returns, benchmark)
    )
    assert UpDownCaptureRatio.from_returns([0.01], [0.02]).compute() is None
    assert UpDownCaptureRatio.from_returns(
        [0.02, 0.0], [0.01, -0.01]
    ).compute() is None


def test_misalignment_is_rejected_without_mutation() -> None:
    state = UpDownCaptureRatio.from_returns([0.01, -0.02], [0.03, -0.01])
    original_value = state.value
    original_length = len(state)
    with pytest.raises(ValueError, match="equal length"):
        state.extend([0.03, 0.04], [0.02])
    assert len(state) == original_length
    assert state.value == original_value


def test_invalid_values_and_ambiguous_factory_are_rejected() -> None:
    with pytest.raises(ValueError):
        UpDownCaptureRatio.from_returns([np.nan], [0.01], nan_policy="raise")
    with pytest.raises(ValueError):
        UpDownCaptureRatio.from_returns([np.inf], [0.01])
    with pytest.raises(ValueError):
        UpDownCaptureRatio.from_returns([0.01, -0.02], [0.01])
    with pytest.raises(ValueError):
        UpDownCaptureRatio.from_returns([0.01], [-0.02], periods_per_year=0.0)
    with pytest.raises(TypeError):
        UpDownCaptureRatio()
