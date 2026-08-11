from __future__ import annotations

import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs

from taflow.metrics.longest_winning_streak import LongestWinningStreak


def _quantstats_consecutive_wins(values: np.ndarray) -> float | None:
    """Evaluate QuantStats 0.0.81 after TAFlow-compatible NaN omission."""
    usable = values[~np.isnan(values)]
    if usable.size == 0:
        return None
    return float(
        qs.consecutive_wins(
            pd.Series(usable),
            aggregate=None,
            compounded=True,
            prepare_returns=False,
        )
    )


@pytest.mark.parametrize(
    "returns",
    [
        np.array([0.01, 0.02, 0.0, 0.03, 0.04, 0.05, -0.01, 0.06]),
        np.array([-0.20]),
        np.array([0.20]),
        np.zeros(32),
        np.linspace(-0.04, 0.05, 101),
        np.array([0.02, 0.02, -0.01, -0.01, 0.0, 0.02]),
        np.array([np.nan, 0.02, 0.03, np.nan, 0.01, -0.01]),
        np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
    ],
)
def test_longest_winning_streak_matches_quantstats(returns: np.ndarray) -> None:
    actual = LongestWinningStreak.from_returns(returns).compute()
    expected = _quantstats_consecutive_wins(returns)
    assert actual == expected


def test_longest_winning_streak_zero_and_negative_break_runs() -> None:
    values = [0.01, 0.02, 0.0, 0.03, 0.04, -0.01, 0.05]
    assert LongestWinningStreak.from_returns(values).compute() == 2.0
    assert LongestWinningStreak.from_returns(values).compute() == (
        _quantstats_consecutive_wins(np.asarray(values))
    )


def test_longest_winning_streak_raw_factories_and_lifecycle() -> None:
    values = np.array([100.0, 20.0, -40.0, 50.0, 60.0, 70.0, 0.0])
    expected = _quantstats_consecutive_wins(values)
    assert LongestWinningStreak.from_pnl(values).compute() == expected
    assert LongestWinningStreak.from_trades(values).compute() == expected

    state = LongestWinningStreak.from_trades([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value == 1.0
    assert state.extend(values[1:3]) is state
    assert state.extend(values[3:]) is state
    assert state.compute() == expected
    assert len(state) == len(values)
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == expected


def test_longest_winning_streak_missing_and_invalid_semantics() -> None:
    state = LongestWinningStreak.from_returns([0.01, np.nan, 0.02])
    assert len(state) == 2
    assert state.compute() == 2.0
    assert LongestWinningStreak.from_returns([]).compute() is None
    assert LongestWinningStreak.from_returns([0.0, -0.01]).compute() == 0.0
    with pytest.raises(ValueError):
        LongestWinningStreak.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        LongestWinningStreak.from_returns([np.inf])
    with pytest.raises(ValueError):
        LongestWinningStreak.from_returns([-2.0])
    assert LongestWinningStreak.from_pnl([-2.0]).compute() == 0.0
    assert LongestWinningStreak.from_trades([-2.0]).compute() == 0.0


def test_longest_winning_streak_column_and_factory_contract() -> None:
    frame = pd.DataFrame({"strategy": [0.1, 0.2], "other": [9.0, 8.0]})
    assert (
        LongestWinningStreak.from_returns(frame, column="strategy").compute() == 2.0
    )
    with pytest.raises(ValueError):
        LongestWinningStreak.from_returns(frame)
    with pytest.raises(TypeError):
        LongestWinningStreak()
