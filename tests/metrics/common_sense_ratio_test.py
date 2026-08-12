from __future__ import annotations
import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics.common_sense_ratio import CommonSenseRatio

def _quantstats_component_oracle(returns: np.ndarray) -> float | None:
    """Compose pinned QuantStats components without preparation heuristics."""
    usable = returns[~np.isnan(returns)]
    if usable.size == 0 or not np.any(usable < 0.0):
        return None
    series = pd.Series(usable)
    profit_factor = float(qs.profit_factor(series, prepare_returns=False))
    tail_ratio = float(qs.tail_ratio(series, cutoff=0.95, prepare_returns=False))
    result = profit_factor * tail_ratio
    return float(result) if np.isfinite(result) else None

@pytest.mark.parametrize('returns', [np.array([-0.1, -0.04, -0.01, 0.0, 0.02, 0.05, 0.12]), np.array([-0.2]), np.linspace(-0.1, 0.08, 101), np.array([-0.05, -0.05, -0.01, 0.02, 0.02, 0.07]), np.array([np.nan, -0.08, 0.01, np.nan, -0.02, 0.03]), np.random.default_rng(20260811).normal(0.0004, 0.012, 513)])
def test_common_sense_ratio_matches_quantstats_components(returns: np.ndarray) -> None:
    actual = CommonSenseRatio().from_returns(returns).compute()
    expected = _quantstats_component_oracle(returns)
    assert expected is not None
    assert actual == pytest.approx(expected, rel=1e-12, abs=1e-14)

def test_common_sense_ratio_matches_direct_quantstats_on_clean_returns() -> None:
    returns = pd.Series([-0.1, -0.04, -0.01, 0.0, 0.02, 0.05, 0.12])
    expected = float(qs.common_sense_ratio(returns, prepare_returns=False))
    assert CommonSenseRatio().from_returns(returns).compute() == pytest.approx(expected)

def test_common_sense_ratio_lifecycle_and_missing_are_invariant() -> None:
    returns = np.array([-0.1, np.nan, -0.04, -0.01, 0.0, 0.02, 0.05, 0.12])
    expected = CommonSenseRatio().from_returns(returns).compute()
    state = CommonSenseRatio().from_returns([])
    assert state.value is None
    assert state.append(returns[0]) is state
    assert state.extend(returns[1:4]) is state
    assert state.extend(returns[4:]) is state
    assert state.compute() == pytest.approx(expected)
    assert len(state) == 7
    assert state.reset() is state
    assert state.value is None
    assert len(state) == 0
    assert state.extend(returns).compute() == pytest.approx(expected)

def test_common_sense_ratio_undefined_and_invalid_edges() -> None:
    assert CommonSenseRatio().from_returns([]).compute() is None
    assert CommonSenseRatio().from_returns([0.0, 0.0]).compute() is None
    assert CommonSenseRatio().from_returns([0.01, 0.02]).compute() is None
    assert CommonSenseRatio().from_returns([-0.01, -0.02]).compute() == 0.0
    with pytest.raises(ValueError):
        CommonSenseRatio(nan_policy='raise').from_returns([np.nan])
    with pytest.raises(ValueError):
        CommonSenseRatio().from_returns([np.inf])
    with pytest.raises(ValueError):
        CommonSenseRatio().from_returns([-1.01])
    unbound = CommonSenseRatio()
    with pytest.raises(ValueError):
        unbound.append(0.01)

def test_common_sense_ratio_column_selection() -> None:
    frame = pd.DataFrame({'strategy': [-0.1, 0.02, 0.03], 'other': [0.5, 0.6, 0.7]})
    expected = CommonSenseRatio().from_returns(frame['strategy']).compute()
    assert CommonSenseRatio().from_returns(frame, column='strategy').compute() == pytest.approx(expected)
    with pytest.raises(ValueError):
        CommonSenseRatio().from_returns(frame)
