from __future__ import annotations
import numpy as np
import pytest
from taflow.metrics.average_drawdown import AverageDrawdown
PERFORMANCE_ANALYTICS_2_1_0_SHA256 = 'fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d'

def _performance_analytics_source_convention(returns: np.ndarray) -> float | None:
    """Translate pinned Drawdowns/findDrawdowns source for specification parity."""
    usable = returns[~np.isnan(returns)]
    if len(usable) == 0:
        return None
    wealth = np.cumprod(1.0 + usable)
    peaks = np.maximum.accumulate(np.r_[1.0, wealth])[1:]
    drawdowns = wealth / peaks - 1.0
    episode_depths: list[float] = []
    current_depth: float | None = None
    for drawdown in drawdowns:
        if drawdown < 0.0:
            current_depth = float(drawdown) if current_depth is None else min(current_depth, float(drawdown))
        elif current_depth is not None:
            episode_depths.append(abs(current_depth))
            current_depth = None
    if current_depth is not None:
        episode_depths.append(abs(current_depth))
    return float(np.mean(episode_depths)) if episode_depths else 0.0

@pytest.mark.parametrize('returns', [np.array([0.25, -0.2, 0.0, 0.1, -0.5, 1.0, -0.1]), np.array([-0.2]), np.array([0.2]), np.zeros(32), np.linspace(-0.04, 0.05, 101), np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_average_drawdown_matches_pinned_performanceanalytics_source_convention(returns: np.ndarray) -> None:
    assert len(PERFORMANCE_ANALYTICS_2_1_0_SHA256) == 64
    actual = AverageDrawdown().from_returns(returns).compute()
    expected = _performance_analytics_source_convention(returns)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_average_drawdown_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.25, -0.2, 0.25, 0.1, -0.5, 1.0, -0.1])
    expected = AverageDrawdown().from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert AverageDrawdown().from_equity(equity).compute() == pytest.approx(expected)
    assert AverageDrawdown().from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert AverageDrawdown().from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = AverageDrawdown().from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.extend(returns[1:3]) is state
    assert state.extend(returns[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_average_drawdown_episode_boundaries_and_current_episode() -> None:
    state = AverageDrawdown().from_returns([0.25, -0.2])
    assert state.compute() == pytest.approx(0.2)
    assert state.append(0.25).compute() == pytest.approx(0.2)
    assert state.extend([0.1, -0.5]).compute() == pytest.approx(0.35)
    assert AverageDrawdown().from_returns([0.1, 0.0, 0.2]).compute() == 0.0

def test_average_drawdown_missing_and_invalid_values() -> None:
    filtered = np.array([0.25, -0.2, 0.25, -0.1])
    state = AverageDrawdown().from_returns([0.25, np.nan, -0.2, 0.25, -0.1])
    assert len(state) == len(filtered)
    assert state.compute() == pytest.approx(_performance_analytics_source_convention(filtered))
    with pytest.raises(ValueError):
        AverageDrawdown(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        AverageDrawdown().from_returns([np.inf])
    with pytest.raises(ValueError):
        AverageDrawdown().from_returns([-1.01])

def test_average_drawdown_requires_semantic_input_method() -> None:
    metric = AverageDrawdown()
    with pytest.raises(ValueError):
        metric.append(0.01)
