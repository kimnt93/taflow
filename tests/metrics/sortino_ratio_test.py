from __future__ import annotations

import empyrical
import numpy as np
import pytest

from taflow.metrics.sortino_ratio import SortinoRatio


def _oracle(
    returns: np.ndarray,
    periods_per_year: float,
    annual_required_return: float,
) -> float:
    period_required_return = np.expm1(
        np.log1p(annual_required_return) / periods_per_year
    )
    return float(
        empyrical.sortino_ratio(
            returns,
            required_return=period_required_return,
            annualization=periods_per_year,
        )
    )


@pytest.mark.parametrize(
    ("returns", "periods_per_year", "annual_required_return"),
    [
        (np.array([0.01, -0.01]), 252.0, 0.0),
        (np.array([0.10, -0.20, 0.05]), 12.0, 0.03),
        (np.linspace(-0.04, 0.05, 101), 52.0, 0.08),
        (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 252.0, 0.0),
        (
            np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
            252.0,
            0.045,
        ),
    ],
)
def test_sortino_ratio_matches_empyrical(
    returns: np.ndarray,
    periods_per_year: float,
    annual_required_return: float,
) -> None:
    actual = SortinoRatio.from_returns(
        returns,
        periods_per_year=periods_per_year,
        annual_required_return=annual_required_return,
    ).compute()
    expected = _oracle(returns, periods_per_year, annual_required_return)
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)


def test_sortino_ratio_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    kwargs = {"periods_per_year": 12.0, "annual_required_return": 0.03}
    expected = SortinoRatio.from_returns(returns, **kwargs).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    pnl = np.array([10.0, -22.0, 4.4])

    assert SortinoRatio.from_equity(equity, **kwargs).compute() == pytest.approx(
        expected
    )
    assert SortinoRatio.from_pnl(
        pnl, initial_equity=100.0, **kwargs
    ).compute() == pytest.approx(expected)
    assert SortinoRatio.from_log_returns(
        np.log1p(returns), **kwargs
    ).compute() == pytest.approx(expected)

    state = SortinoRatio.from_returns([], **kwargs)
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.value is None
    assert state.extend(returns[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(returns)
    assert state.reset() is state
    assert len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)


def test_sortino_ratio_missing_zero_downside_and_invalid_values() -> None:
    assert SortinoRatio.from_returns([0.25]).compute() is None
    assert SortinoRatio.from_returns([0.01, 0.02]).compute() is None
    assert len(SortinoRatio.from_returns([np.nan, -0.01, 0.02])) == 2
    with pytest.raises(ValueError):
        SortinoRatio.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        SortinoRatio.from_returns([np.inf])
    with pytest.raises(ValueError):
        SortinoRatio.from_returns([0.01, -0.02], periods_per_year=0.0)
    with pytest.raises(ValueError):
        SortinoRatio.from_returns(
            [0.01, -0.02], annual_required_return=-1.0
        )


def test_sortino_ratio_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        SortinoRatio()
