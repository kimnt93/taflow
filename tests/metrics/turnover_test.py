from __future__ import annotations

import numpy as np
import pytest

from taflow.metrics.turnover import Turnover


@pytest.mark.parametrize(
    "weights",
    [
        np.array([0.0, 0.5, -0.25, 0.25]),
        np.zeros(32),
        np.linspace(-1.5, 2.0, 101),
        np.random.default_rng(20260812).normal(0.25, 0.7, 513),
        np.array([np.nan, 0.0, 0.5, np.nan, -0.25]),
    ],
)
def test_turnover_matches_numpy_oracle(weights: np.ndarray) -> None:
    usable = weights[~np.isnan(weights)]
    expected = None if len(usable) < 2 else float(np.mean(np.abs(np.diff(usable))))
    actual = Turnover.from_weights(weights).compute()
    if expected is None:
        assert actual is None
    else:
        assert actual == pytest.approx(expected, rel=1e-14, abs=1e-15)


def test_turnover_lifecycle_is_invariant() -> None:
    weights = np.array([0.0, 0.4, 0.1, -0.2, 0.0])
    expected = Turnover.from_weights(weights).compute()
    state = Turnover.from_weights([])
    assert state.append(weights[0]) is state and state.value is None
    assert state.extend(weights[1:3]) is state
    assert state.extend(weights[3:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == len(weights)
    assert state.compute() == state.compute()
    assert state.reset() is state and len(state) == 0
    assert state.extend(weights).compute() == pytest.approx(expected)


def test_turnover_validation_and_semantic_factory() -> None:
    assert Turnover.from_weights([]).compute() is None
    assert Turnover.from_weights([0.5]).compute() is None
    with pytest.raises(ValueError):
        Turnover.from_weights([np.nan], nan_policy="raise")
    with pytest.raises(ValueError):
        Turnover.from_weights([np.inf])
    with pytest.raises(TypeError):
        Turnover()
