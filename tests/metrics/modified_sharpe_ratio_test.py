from __future__ import annotations

from statistics import NormalDist

import numpy as np
import pytest

from taflow.metrics.modified_sharpe_ratio import ModifiedSharpeRatio


PERFORMANCE_ANALYTICS_2_1_0_SHA256 = (
    "fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d"
)


def _performanceanalytics_modified_sharpe(
    returns: np.ndarray,
    periods_per_year: float,
    annual_risk_free_rate: float,
    confidence_level: float,
) -> float | None:
    usable = returns[~np.isnan(returns)]
    if len(usable) < 2:
        return None
    period_rf = np.expm1(np.log1p(annual_risk_free_rate) / periods_per_year)
    excess = usable - period_rf
    mean = float(np.mean(excess))
    centered = excess - mean
    variance = float(np.mean(centered**2))
    if abs(variance) <= np.sqrt(np.finfo(np.float64).eps):
        skewness = 0.0
        excess_kurtosis = 0.0
    else:
        skewness = float(np.mean(centered**3) / variance**1.5)
        excess_kurtosis = float(np.mean(centered**4) / variance**2 - 3.0)
    z = NormalDist().inv_cdf(1.0 - confidence_level)
    adjusted = (
        z
        + (z**2 - 1.0) * skewness / 6.0
        + (z**3 - 3.0 * z) * excess_kurtosis / 24.0
        - (2.0 * z**3 - 5.0 * z) * skewness**2 / 36.0
    )
    modified_var = -mean - adjusted * np.sqrt(variance)
    if not np.isfinite(modified_var) or modified_var < 0.0:
        return None
    modified_var = min(float(modified_var), 1.0)
    if modified_var == 0.0:
        return None
    return mean / modified_var


@pytest.mark.parametrize(
    ("returns", "periods_per_year", "annual_risk_free_rate", "confidence_level"),
    [
        (np.array([0.02, -0.01, 0.03, -0.025, 0.01, -0.04, 0.03]), 252.0, 0.0, 0.95),
        (np.linspace(-0.04, 0.05, 101), 52.0, -0.01, 0.975),
        (np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), 12.0, 0.04, 0.99),
        (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 252.0, 0.03, 0.95),
        (np.random.default_rng(20260811).normal(0.0004, 0.012, 513), 365.0, 0.05, 0.90),
    ],
)
def test_modified_sharpe_ratio_matches_pinned_source_translation(
    returns: np.ndarray,
    periods_per_year: float,
    annual_risk_free_rate: float,
    confidence_level: float,
) -> None:
    assert len(PERFORMANCE_ANALYTICS_2_1_0_SHA256) == 64
    actual = ModifiedSharpeRatio.from_returns(
        returns, periods_per_year=periods_per_year,
        annual_risk_free_rate=annual_risk_free_rate,
        confidence_level=confidence_level,
    ).compute()
    expected = _performanceanalytics_modified_sharpe(
        returns, periods_per_year, annual_risk_free_rate, confidence_level
    )
    if expected is None:
        assert actual is None
    else:
        assert actual == pytest.approx(expected, rel=2e-8, abs=2e-10)


def test_modified_sharpe_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05, -0.03])
    settings = {"periods_per_year": 12.0, "annual_risk_free_rate": 0.03, "confidence_level": 0.975}
    expected = ModifiedSharpeRatio.from_returns(returns, **settings).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert ModifiedSharpeRatio.from_equity(equity, **settings).compute() == pytest.approx(expected)
    assert ModifiedSharpeRatio.from_pnl(pnl, initial_equity=100.0, **settings).compute() == pytest.approx(expected)
    assert ModifiedSharpeRatio.from_log_returns(np.log1p(returns), **settings).compute() == pytest.approx(expected)

    state = ModifiedSharpeRatio.from_returns([], **settings)
    assert state.append(returns[0]) is state and state.value is None
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state and len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_modified_sharpe_boundaries_and_validation() -> None:
    assert ModifiedSharpeRatio.from_returns([]).compute() is None
    assert ModifiedSharpeRatio.from_returns([-0.01]).compute() is None
    assert ModifiedSharpeRatio.from_returns([-0.01, -0.01]).compute() == pytest.approx(-1.0)
    assert ModifiedSharpeRatio.from_returns([0.01, 0.01]).compute() is None
    assert len(ModifiedSharpeRatio.from_returns([np.nan, 0.02, -0.01])) == 2
    with pytest.raises(ValueError):
        ModifiedSharpeRatio.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        ModifiedSharpeRatio.from_returns([np.inf])
    with pytest.raises(ValueError):
        ModifiedSharpeRatio.from_returns([-1.01])
    with pytest.raises(ValueError):
        ModifiedSharpeRatio.from_returns([0.01, -0.01], confidence_level=0.5)
    with pytest.raises(ValueError):
        ModifiedSharpeRatio.from_returns([0.01, -0.01], periods_per_year=0.0)


def test_modified_sharpe_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        ModifiedSharpeRatio()
