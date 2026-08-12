from __future__ import annotations
import numpy as np
import pytest
from taflow.metrics.conditional_drawdown_at_risk import ConditionalDrawdownAtRisk
PERFORMANCE_ANALYTICS_2_1_0_SHA256 = 'fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d'
RISKFOLIO_LIB_7_3_0_SHA256 = '73ab9400691e0ca4258ba1b3d4939018c67809955504807b0559904405246bdf'

def _performanceanalytics_discrete(returns: np.ndarray, confidence: float) -> float | None:
    """Independent translation of pinned CDD/findDrawdowns source."""
    usable = returns[~np.isnan(returns)]
    if len(usable) == 0:
        return None
    wealth = np.cumprod(1.0 + usable)
    peaks = np.maximum.accumulate(np.r_[1.0, wealth])[1:]
    drawdowns = wealth / peaks - 1.0
    episodes: list[float] = []
    prior_negative = bool(drawdowns[0] < 0.0)
    trough = float(drawdowns[0])
    for drawdown in drawdowns[1:]:
        negative = bool(drawdown < 0.0)
        if negative == prior_negative:
            trough = min(trough, float(drawdown))
        else:
            episodes.append(trough)
            prior_negative = negative
            trough = float(drawdown)
    episodes.append(trough)
    boundary = float(np.quantile(episodes, 1.0 - confidence, method='linear'))
    tail = np.asarray(episodes)[np.asarray(episodes) <= boundary]
    return -float(np.mean(tail))

@pytest.mark.parametrize('confidence', [0.5, 0.75, 0.9, 0.95, 0.99])
@pytest.mark.parametrize('returns', [np.array([-0.2]), np.array([0.2]), np.zeros(32), np.array([0.25, -0.2, 0.25, 0.1, -0.5, 1.0, -0.1]), np.array([0.25, -0.2, 0.25, 0.25, -0.2]), np.linspace(-0.04, 0.05, 101), np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), np.random.default_rng(20260812).normal(0.0004, 0.012, 513)])
def test_matches_pinned_performanceanalytics_discrete_source(returns: np.ndarray, confidence: float) -> None:
    assert len(PERFORMANCE_ANALYTICS_2_1_0_SHA256) == 64
    assert len(RISKFOLIO_LIB_7_3_0_SHA256) == 64
    actual = ConditionalDrawdownAtRisk(confidence=confidence).from_returns(returns).compute()
    expected = _performanceanalytics_discrete(returns, confidence)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_input_methods_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.25, -0.2, 0.25, 0.1, -0.5, 1.0, -0.1])
    expected = ConditionalDrawdownAtRisk(confidence=0.75).from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)
    assert ConditionalDrawdownAtRisk(confidence=0.75).from_equity(equity).compute() == pytest.approx(expected)
    assert ConditionalDrawdownAtRisk(confidence=0.75).from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    assert ConditionalDrawdownAtRisk(confidence=0.75).from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    state = ConditionalDrawdownAtRisk(confidence=0.75).from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.extend(returns[1:3]) is state
    assert state.extend(returns[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_current_episode_zero_paths_and_boundary_ties() -> None:
    state = ConditionalDrawdownAtRisk().from_returns([0.25, -0.2])
    assert state.compute() == pytest.approx(0.2)
    assert state.append(0.25).compute() == pytest.approx(0.2)
    assert ConditionalDrawdownAtRisk().from_returns([0.1, 0.0, 0.2]).compute() == 0.0
    assert ConditionalDrawdownAtRisk().from_returns([0.25, -0.2, 0.25, 0.25, -0.2]).compute() == pytest.approx(0.2)

def test_missing_and_invalid_values_are_enforced() -> None:
    returns = np.array([0.25, np.nan, -0.2, 0.25, -0.1])
    usable = returns[~np.isnan(returns)]
    state = ConditionalDrawdownAtRisk().from_returns(returns)
    assert len(state) == len(usable)
    assert state.compute() == pytest.approx(_performanceanalytics_discrete(usable, 0.95))
    for confidence in [0.0, 1.0, -0.1, 1.1, np.nan, np.inf]:
        with pytest.raises(ValueError):
            ConditionalDrawdownAtRisk(confidence=confidence).from_returns([0.01])
    with pytest.raises(ValueError):
        ConditionalDrawdownAtRisk(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        ConditionalDrawdownAtRisk().from_returns([np.inf])
    with pytest.raises(ValueError):
        ConditionalDrawdownAtRisk().from_returns([-1.01])

def test_requires_semantic_input_method() -> None:
    metric = ConditionalDrawdownAtRisk()
    with pytest.raises(ValueError):
        metric.append(0.01)
