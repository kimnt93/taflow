from __future__ import annotations

import empyrical
import numpy as np
import pytest

from taflow.metrics.tail_ratio import TailRatio


@pytest.mark.parametrize(
    "returns",
    [
        np.array([-0.03]),
        np.full(32, 0.01),
        np.linspace(-0.10, 0.08, 101),
        np.array([-0.05, -0.05, -0.01, 0.02, 0.02, 0.07]),
        np.random.default_rng(20260811).normal(0.0004, 0.012, 513),
    ],
)
def test_tail_ratio_matches_empyrical(returns: np.ndarray) -> None:
    actual = TailRatio.from_returns(returns).compute()
    expected = float(empyrical.tail_ratio(returns))
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-15)


def test_tail_ratio_omits_nan_before_oracle_comparison() -> None:
    returns = np.array([np.nan, -0.08, 0.01, np.nan, -0.02, 0.03])
    usable = returns[~np.isnan(returns)]
    actual = TailRatio.from_returns(returns).compute()
    expected = float(empyrical.tail_ratio(usable))
    assert actual == pytest.approx(expected, rel=1e-13, abs=1e-15)
    assert len(TailRatio.from_returns(returns)) == len(usable)


def test_tail_ratio_factories_and_lifecycle_are_invariant() -> None:
    returns = np.array([0.10, -0.20, 0.05])
    expected = TailRatio.from_returns(returns).compute()
    equity = np.array([100.0, 110.0, 88.0, 92.4])
    pnl = np.array([10.0, -22.0, 4.4])

    assert TailRatio.from_equity(equity).compute() == pytest.approx(expected)
    assert TailRatio.from_pnl(pnl, initial_equity=100.0).compute() == pytest.approx(
        expected
    )
    assert TailRatio.from_log_returns(np.log1p(returns)).compute() == pytest.approx(
        expected
    )

    scalar = TailRatio.from_returns([])
    for value in returns:
        assert scalar.append(value) is scalar
    chunked = TailRatio.from_returns(returns[:1])
    assert chunked.extend(returns[1:]) is chunked
    assert scalar.compute() == pytest.approx(expected)
    assert chunked.compute() == pytest.approx(expected)
    assert scalar.value == pytest.approx(expected)
    assert len(scalar) == len(returns)
    assert scalar.reset() is scalar
    assert len(scalar) == 0
    assert scalar.compute() is None
    assert scalar.extend(returns).compute() == pytest.approx(expected)


def test_tail_ratio_zero_lower_tail_is_undefined() -> None:
    assert np.isnan(empyrical.tail_ratio(np.zeros(32)))
    assert TailRatio.from_returns(np.zeros(32)).compute() is None


def test_tail_ratio_rejects_invalid_inputs() -> None:
    with pytest.raises(ValueError):
        TailRatio.from_returns([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        TailRatio.from_returns([np.inf])
    with pytest.raises(ValueError):
        TailRatio.from_returns([-1.01])
    with pytest.raises(ValueError):
        TailRatio.from_pnl([1.0], initial_equity=0.0)


def test_tail_ratio_requires_semantic_factory() -> None:
    with pytest.raises(TypeError):
        TailRatio()
