from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.exposure import Exposure

def _quantstats_exposure(values: np.ndarray) -> float | None:
    """Run the pinned oracle after TAFlow's package-wide missing policy."""
    usable = values[~np.isnan(values)]
    if len(usable) == 0:
        return None
    series = pd.Series(usable, index=pd.date_range('2000-01-01', periods=len(usable), freq='D'))
    return float(qs.exposure(series, prepare_returns=False))

@pytest.mark.parametrize('returns', [np.array([0.01, 0.0, 0.02, 0.0, 0.03]), np.array([0.01, 0.0, 0.0]), np.zeros(32), np.linspace(-0.04, 0.05, 101), np.array([0.02, -0.0, -0.01, 0.0, 0.02]), np.array([np.nan, 0.02, 0.0, np.nan, -0.01]), np.random.default_rng(20260812).choice([0.0, -0.01, 0.01], size=513, p=[0.4, 0.3, 0.3])])
def test_return_proxy_matches_quantstats_0_0_81(returns: np.ndarray) -> None:
    actual = Exposure().from_returns(returns).compute()
    expected = _quantstats_exposure(returns)
    assert actual == pytest.approx(expected, rel=0.0, abs=1e-15)

def test_ceiling_to_percentage_point_is_intentionally_retained() -> None:
    assert Exposure().from_returns([0.01, 0.0, 0.0]).compute() == pytest.approx(0.34)
    assert Exposure().from_returns([0.01] + [0.0] * 100).compute() == pytest.approx(0.01)

def test_explicit_positions_accept_short_and_leveraged_states() -> None:
    positions = np.array([0.0, 1.0, -1.0, 0.0, 0.5, 2.0])
    assert Exposure().from_positions(positions).compute() == pytest.approx(0.67)
    assert len(Exposure().from_positions(positions)) == len(positions)

@pytest.mark.parametrize('input_method_name', ['from_returns', 'from_positions'])
def test_lifecycle_missing_reset_and_cached_compute_are_invariant(input_method_name: str) -> None:
    values = np.array([0.01, np.nan, 0.0, -0.02, 0.0, 0.03])
    expected = getattr(Exposure(), input_method_name)(values).compute()
    state = getattr(Exposure(), input_method_name)([])
    assert state.value is None
    assert state.append(values[0]) is state
    assert state.extend(values[1:3]) is state
    assert state.extend(values[3:]) is state
    assert state.compute() == expected
    assert state.compute() == expected
    assert len(state) == 5
    assert state.reset() is state
    assert state.value is None
    assert len(state) == 0
    assert state.extend(values).compute() == expected

def test_missing_invalid_and_input_method_contract() -> None:
    assert Exposure().from_returns([]).compute() is None
    assert Exposure().from_returns([np.nan]).compute() is None
    assert Exposure().from_positions([-2.0]).compute() == 1.0
    with pytest.raises(ValueError, match='NaN'):
        Exposure(nan_policy='raise').from_positions([np.nan])
    with pytest.raises(ValueError):
        Exposure().from_positions([np.inf])
    with pytest.raises(ValueError):
        Exposure().from_returns([-1.01])
    unbound = Exposure()
    with pytest.raises(ValueError):
        unbound.append(0.01)
