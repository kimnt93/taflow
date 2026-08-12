from __future__ import annotations

import numpy as np
import pytest
from scipy.stats import kurtosis, norm, skew

from taflow.metrics.deflated_sharpe_ratio import DeflatedSharpeRatio


VECTORBT_0_28_5_COMMIT = "993ceca7116fc8e55f4cd3a36fe43d83dab62b27"


def _vectorbt_dsr(returns: np.ndarray, ppy: float, rf: float, trials: int, annual_variance: float) -> float | None:
    usable = returns[~np.isnan(returns)]
    if len(usable) < 4: return None
    excess = usable - np.expm1(np.log1p(rf) / ppy)
    std = float(np.std(excess, ddof=1))
    if std == 0.0: return None
    sr = float(np.mean(excess) / std)
    gamma = np.euler_gamma
    expected_maximum = np.sqrt(annual_variance / ppy) * ((1.0 - gamma) * norm.ppf(1.0 - 1.0 / trials) + gamma * norm.ppf(1.0 - 1.0 / (trials * np.e)))
    adjustment = 1.0 - float(skew(excess, bias=False)) * sr + (float(kurtosis(excess, fisher=False, bias=False)) - 1.0) * sr**2 / 4.0
    if not np.isfinite(adjustment) or adjustment <= 0.0: return None
    return float(norm.cdf((sr - expected_maximum) * np.sqrt(len(usable) - 1.0) / np.sqrt(adjustment)))


@pytest.mark.parametrize(("returns", "ppy", "rf", "trials", "variance"), [
    (np.array([0.02, -0.01, 0.03, -0.025, 0.01, -0.04, 0.015]), 252.0, 0.0, 20, 0.64),
    (np.linspace(-0.04, 0.05, 101), 52.0, -0.01, 5, 0.2),
    (np.random.default_rng(91).standard_t(5, 513) * 0.012, 365.0, 0.05, 100, 1.4),
    (np.array([np.nan, 0.02, -0.03, np.nan, 0.01, 0.04]), 12.0, 0.03, 2, 0.0),
])
def test_deflated_sharpe_matches_vectorbt(returns: np.ndarray, ppy: float, rf: float, trials: int, variance: float) -> None:
    assert len(VECTORBT_0_28_5_COMMIT) == 40
    actual = DeflatedSharpeRatio.from_returns(returns, number_of_trials=trials, annual_sharpe_ratio_variance=variance, periods_per_year=ppy, annual_risk_free_rate=rf).compute()
    expected = _vectorbt_dsr(returns, ppy, rf, trials, variance)
    assert actual == pytest.approx(expected, rel=2e-9, abs=2e-10)


def test_deflated_sharpe_factories_and_lifecycle() -> None:
    returns = np.array([0.10, -0.20, 0.05, -0.03, 0.08, -0.01])
    settings = dict(number_of_trials=8, annual_sharpe_ratio_variance=0.25, periods_per_year=12.0, annual_risk_free_rate=0.03)
    expected = DeflatedSharpeRatio.from_returns(returns, **settings).compute()
    equity = 100 * np.r_[1.0, np.cumprod(1 + returns)]
    assert DeflatedSharpeRatio.from_equity(equity, **settings).compute() == pytest.approx(expected)
    assert DeflatedSharpeRatio.from_pnl(np.diff(equity), initial_equity=100, **settings).compute() == pytest.approx(expected)
    assert DeflatedSharpeRatio.from_log_returns(np.log1p(returns), **settings).compute() == pytest.approx(expected)
    state = DeflatedSharpeRatio.from_returns([], **settings)
    for value in returns: assert state.append(value) is state
    assert state.compute() == pytest.approx(expected)
    assert state.compute() == state.compute()
    assert state.reset() is state and len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_deflated_sharpe_edges_and_validation() -> None:
    kwargs = dict(number_of_trials=2, annual_sharpe_ratio_variance=0.1)
    assert DeflatedSharpeRatio.from_returns([0.01, -0.01, 0.02], **kwargs).compute() is None
    assert DeflatedSharpeRatio.from_returns([0.01] * 8, **kwargs).compute() is None
    with pytest.raises(ValueError): DeflatedSharpeRatio.from_returns([0.01] * 4, number_of_trials=1, annual_sharpe_ratio_variance=0.1)
    with pytest.raises(ValueError): DeflatedSharpeRatio.from_returns([0.01] * 4, number_of_trials=2, annual_sharpe_ratio_variance=-0.1)
    with pytest.raises(TypeError): DeflatedSharpeRatio()
