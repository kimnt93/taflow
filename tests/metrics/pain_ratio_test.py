from __future__ import annotations

import numpy as np
import pytest

from taflow.metrics.pain_ratio import PainRatio


PERFORMANCE_ANALYTICS_2_1_0_SHA256 = (
    "fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d"
)


def _performance_analytics_source_convention(
    returns: np.ndarray,
    periods_per_year: float,
    annual_risk_free_rate: float,
) -> float | None:
    """Translate pinned PainRatio, PainIndex, and DrawdownPeak source."""
    usable = returns[~np.isnan(returns)]
    if len(usable) == 0:
        return None
    wealth = np.cumprod(1.0 + usable)
    peaks = np.maximum.accumulate(np.r_[1.0, wealth])[1:]
    pain_index = float(np.mean(np.abs(wealth / peaks - 1.0)))
    if pain_index == 0.0:
        return None
    annualized_return = float(
        np.prod(1.0 + usable) ** (periods_per_year / len(usable)) - 1.0
    )
    return (annualized_return - annual_risk_free_rate) / pain_index


@pytest.mark.parametrize(
    ("returns", "periods_per_year", "annual_risk_free_rate"),
    [
        (np.array([0.02, -0.01, 0.015, -0.03, 0.01]), 252.0, 0.0),
        (np.array([-0.20]), 12.0, 0.03),
        (np.linspace(-0.04, 0.05, 101), 52.0, -0.01),
        (np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), 12.0, 0.04),
        (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 252.0, 0.03),
        (
            np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
            365.0,
            0.05,
        ),
    ],
)
def test_pain_ratio_matches_pinned_performanceanalytics_source_convention(
    returns: np.ndarray,
    periods_per_year: float,
    annual_risk_free_rate: float,
) -> None:
    # This is pinned source-specification parity, not an executable R match.
    assert len(PERFORMANCE_ANALYTICS_2_1_0_SHA256) == 64
    actual = PainRatio.from_returns(
        returns,
        periods_per_year=periods_per_year,
        annual_risk_free_rate=annual_risk_free_rate,
    ).compute()
    expected = _performance_analytics_source_convention(
        returns, periods_per_year, annual_risk_free_rate
    )
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_pain_ratio_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05, -0.25, 0.1])
    settings = {"periods_per_year": 12.0, "annual_risk_free_rate": 0.03}
    expected = PainRatio.from_returns(returns, **settings).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)

    assert PainRatio.from_equity(equity, **settings).compute() == pytest.approx(
        expected
    )
    assert PainRatio.from_pnl(
        pnl, initial_equity=100.0, **settings
    ).compute() == pytest.approx(expected)
    assert PainRatio.from_log_returns(
        np.log1p(returns), **settings
    ).compute() == pytest.approx(expected)

    state = PainRatio.from_returns([], **settings)
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:3]) is state
    assert state.extend(returns[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_pain_ratio_freezes_annual_risk_free_subtraction_and_pain_divisor() -> None:
    returns = np.array([0.10, -0.20])
    pain = 0.20 / 2.0
    annualized = np.prod(1.0 + returns) ** (12.0 / 2.0) - 1.0
    expected = (annualized - 0.03) / pain
    assert PainRatio.from_returns(
        returns, periods_per_year=12.0, annual_risk_free_rate=0.03
    ).compute() == pytest.approx(expected)


def test_pain_ratio_boundaries_missing_and_invalid_values() -> None:
    assert PainRatio.from_returns([]).compute() is None
    assert PainRatio.from_returns([0.25]).compute() is None
    assert PainRatio.from_returns([0.0, 0.0]).compute() is None
    assert PainRatio.from_returns([-1.0], periods_per_year=1.0).compute() == -1.0
    assert len(PainRatio.from_returns([np.nan, 0.01, -0.02])) == 2
    with pytest.raises(ValueError):
        PainRatio.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        PainRatio.from_returns([np.inf])
    with pytest.raises(ValueError):
        PainRatio.from_returns([-1.01])
    with pytest.raises(ValueError):
        PainRatio.from_returns([0.01, -0.02], periods_per_year=0.0)
    with pytest.raises(ValueError):
        PainRatio.from_returns(
            [0.01, -0.02], annual_risk_free_rate=-1.0
        )


def test_pain_ratio_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        PainRatio()
