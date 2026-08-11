from __future__ import annotations

import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs

from taflow.metrics.gain_to_pain_ratio import GainToPainRatio


def quantstats_gain_to_pain_ratio(returns: np.ndarray) -> float | None:
    """Evaluate QuantStats 0.0.81 with one observation per daily bucket."""
    usable = returns[~np.isnan(returns)]
    series = pd.Series(
        usable,
        index=pd.date_range("2000-01-01", periods=len(usable), freq="D"),
    )
    expected = float(qs.gain_to_pain_ratio(series, rf=0.0, resolution="D"))
    return expected if np.isfinite(expected) else None


@pytest.mark.parametrize(
    "returns",
    [
        np.array([0.10, -0.20]),
        np.linspace(-0.04, 0.05, 101),
        np.array([0.15, -0.1, -0.1, 0.25, -0.3, 0.1]),
        np.array([0.2, -0.2, 0.25, -0.2, 0.25, -0.2]),
        np.array([np.nan, 0.02, -0.03, np.nan, 0.01]),
        np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
    ],
)
def test_gain_to_pain_ratio_matches_quantstats(returns: np.ndarray) -> None:
    actual = GainToPainRatio.from_returns(returns).compute()
    expected = quantstats_gain_to_pain_ratio(returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_gain_to_pain_ratio_freezes_net_numerator_and_input_resolution() -> None:
    # QuantStats 0.0.81 uses net sum / absolute negative sum. The positive-only
    # numerator variant would be 2.0, while the frozen oracle result is 1.0.
    assert GainToPainRatio.from_returns([0.02, -0.01]).compute() == pytest.approx(
        1.0
    )

    returns = np.array([0.03, -0.01, 0.02, -0.01])
    assert GainToPainRatio.from_returns(returns).compute() == pytest.approx(
        quantstats_gain_to_pain_ratio(returns)
    )


def test_gain_to_pain_ratio_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05, -0.025, 0.1])
    expected = GainToPainRatio.from_returns(returns).compute()
    equity = 100.0 * np.r_[1.0, np.cumprod(1.0 + returns)]
    pnl = np.diff(equity)

    assert GainToPainRatio.from_equity(equity).compute() == pytest.approx(expected)
    assert GainToPainRatio.from_pnl(
        pnl, initial_equity=100.0
    ).compute() == pytest.approx(expected)
    assert GainToPainRatio.from_log_returns(np.log1p(returns)).compute() == pytest.approx(
        expected
    )

    state = GainToPainRatio.from_returns([])
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


def test_gain_to_pain_ratio_boundaries_missing_and_invalid_values() -> None:
    assert GainToPainRatio.from_returns([]).compute() is None
    assert GainToPainRatio.from_returns([0.25]).compute() is None
    assert GainToPainRatio.from_returns([0.0, 0.0]).compute() is None
    assert GainToPainRatio.from_returns([-0.25]).compute() == -1.0
    assert len(GainToPainRatio.from_returns([np.nan, 0.01, -0.02])) == 2
    with pytest.raises(ValueError):
        GainToPainRatio.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        GainToPainRatio.from_returns([np.inf])
    with pytest.raises(ValueError):
        GainToPainRatio.from_returns([-1.01])


def test_gain_to_pain_ratio_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        GainToPainRatio()
