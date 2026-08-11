"""External correctness and lifecycle tests for GrossProfit."""

from __future__ import annotations

import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs

from taflow.metrics.gross_profit import GrossProfit


def numpy_gross_profit(values: np.ndarray) -> float | None:
    """Return the independent strictly-positive NumPy sum after NaN omission."""
    filtered = values[~np.isnan(values)]
    if filtered.size == 0:
        return None
    return float(np.sum(filtered[filtered > 0.0]))


@pytest.mark.parametrize(
    "values",
    [
        np.array([100.0, -40.0, 0.0, 20.0, -10.0]),
        np.array([0.0, -20.0, -10.0]),
        np.array([25.0]),
        np.linspace(-100.0, 100.0, 101),
        np.array([np.nan, 100.0, -20.0, np.nan, 25.0]),
        np.random.default_rng(20260811).normal(40.0, 500.0, 513),
    ],
)
def test_gross_profit_matches_numpy(values: np.ndarray) -> None:
    expected = numpy_gross_profit(values)
    actual = GrossProfit.from_pnl(values).compute()
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-12)


def test_gross_profit_matches_quantstats_profit_factor_numerator() -> None:
    # The single loss has absolute gross value one, so profit factor equals its
    # strictly non-negative numerator; zero does not change that numerator.
    values = np.array([2.0, 0.0, -1.0, 3.0])
    expected = float(qs.profit_factor(pd.Series(values), prepare_returns=False))
    assert expected == pytest.approx(5.0)
    assert GrossProfit.from_pnl(values).compute() == pytest.approx(expected)


def test_gross_profit_trade_factory_preserves_absolute_values() -> None:
    values = np.array([2500.0, -8000.0, 0.0, 750.0, -20.0])
    assert GrossProfit.from_trades(values).compute() == pytest.approx(3250.0)
    assert len(GrossProfit.from_trades(values)) == values.size


def test_gross_profit_lifecycle_is_chunk_and_reset_invariant() -> None:
    values = np.array([100.0, np.nan, -40.0, 0.0, 20.0, -10.0])
    expected = GrossProfit.from_pnl(values).compute()

    state = GrossProfit.from_pnl([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.value == pytest.approx(100.0)
    assert state.extend(values[1:4]) is state
    assert state.extend(values[4:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == 5
    assert state.reset() is state
    assert len(state) == 0
    assert state.value is None
    assert state.extend(values).compute() == pytest.approx(expected)


def test_gross_profit_boundaries_missing_and_invalid_values() -> None:
    assert GrossProfit.from_pnl([]).compute() is None
    assert GrossProfit.from_pnl([-100.0, 0.0]).compute() == pytest.approx(0.0)
    assert len(GrossProfit.from_pnl([np.nan, 0.0, np.nan])) == 1
    with pytest.raises(ValueError):
        GrossProfit.from_pnl([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        GrossProfit.from_pnl([np.inf])
    with pytest.raises(TypeError):
        GrossProfit.from_pnl([1.0], initial_equity=100.0)


def test_gross_profit_rejects_ambiguous_or_return_domain_construction() -> None:
    with pytest.raises(TypeError):
        GrossProfit()
    assert not hasattr(GrossProfit, "from_returns")
