from __future__ import annotations

import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs

from taflow import UlcerIndex as RollingUlcerIndex
from taflow.metrics import UlcerIndex as ExportedMetricUlcerIndex
from taflow.metrics.ulcer_index import UlcerIndex


def quantstats_ulcer_index(returns: np.ndarray) -> float:
    """Evaluate the pinned oracle with the DatetimeIndex it requires."""
    series = pd.Series(
        returns,
        index=pd.date_range("2000-01-01", periods=len(returns), freq="D"),
    )
    return float(qs.ulcer_index(series))


@pytest.mark.parametrize(
    "returns",
    [
        np.array([0.10, -0.20]),
        np.zeros(32),
        np.linspace(-0.04, 0.05, 101),
        np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]),
        np.array([0.2, -0.2, 0.25, -0.2, 0.25, -0.2]),
        np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
    ],
)
def test_ulcer_index_matches_quantstats(returns: np.ndarray) -> None:
    actual = UlcerIndex.from_returns(returns).compute()
    expected = quantstats_ulcer_index(returns)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_ulcer_index_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05, -0.25, 0.1])
    expected = UlcerIndex.from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)

    assert UlcerIndex.from_equity(equity).compute() == pytest.approx(expected)
    assert UlcerIndex.from_pnl(pnl, initial_equity=100.0).compute() == pytest.approx(
        expected
    )
    assert UlcerIndex.from_log_returns(np.log1p(returns)).compute() == pytest.approx(
        expected
    )

    state = UlcerIndex.from_returns([])
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


def test_ulcer_index_uses_quantstats_phantom_wealth_and_divisor() -> None:
    # Drawdowns are 0, -0.2, 0, so sqrt(sum(dd**2) / (3 - 1)).
    expected = np.sqrt(0.04 / 2.0)
    assert UlcerIndex.from_returns([0.1, -0.2, 0.25]).compute() == pytest.approx(
        expected
    )
    assert UlcerIndex.from_returns([-0.1]).compute() is None


def test_ulcer_index_missing_and_invalid_values() -> None:
    filtered = np.array([0.1, -0.2, 0.05])
    state = UlcerIndex.from_returns([0.1, np.nan, -0.2, 0.05])
    assert len(state) == len(filtered)
    assert state.compute() == pytest.approx(quantstats_ulcer_index(filtered))
    with pytest.raises(ValueError):
        UlcerIndex.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        UlcerIndex.from_returns([np.inf])
    with pytest.raises(ValueError):
        UlcerIndex.from_returns([-1.01])


def test_ulcer_index_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        UlcerIndex()


def test_whole_history_metric_namespace_is_distinct_from_rolling_indicator() -> None:
    assert ExportedMetricUlcerIndex is UlcerIndex
    assert RollingUlcerIndex is not UlcerIndex
