import empyrical
import numpy as np
import pytest
from taflow.metrics import StabilityOfTimeSeries

@pytest.mark.parametrize('values', [np.array([0.01, -0.02, 0.03, 0.015, -0.004]), np.linspace(-0.02, 0.03, 30), np.array([0.01, np.nan, -0.005, 0.02])])
def test_matches_empyrical(values):
    actual = StabilityOfTimeSeries().from_returns(values).compute()
    expected = empyrical.stability_of_timeseries(values)
    assert actual == pytest.approx(expected, rel=5e-11, abs=1e-13)

def test_input_methods_and_lifecycle_are_equivalent():
    returns = np.array([0.01, -0.02, 0.03, 0.015])
    equity = 100.0 * np.cumprod(np.r_[1.0, 1.0 + returns])
    pnl = np.diff(equity)
    expected = StabilityOfTimeSeries().from_returns(returns).compute()
    assert StabilityOfTimeSeries().from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    assert StabilityOfTimeSeries().from_equity(equity).compute() == pytest.approx(expected)
    assert StabilityOfTimeSeries().from_pnl(pnl, initial_capital=100.0).compute() == pytest.approx(expected)
    metric = StabilityOfTimeSeries().from_returns(np.array([], dtype=np.float64))
    metric.append(returns[0]).extend(returns[1:2]).extend(returns[2:])
    assert metric.compute() == pytest.approx(expected)
    assert len(metric) == len(returns)
    assert metric.reset() is metric
    assert metric.extend(returns).compute() == pytest.approx(expected)

def test_validation_and_warmup():
    unbound = StabilityOfTimeSeries()
    with pytest.raises(ValueError):
        unbound.append(0.01)
    assert StabilityOfTimeSeries().from_returns([0.01]).compute() is None
    assert StabilityOfTimeSeries().from_returns([0.01, -1.0]).compute() is None
    with pytest.raises(ValueError):
        StabilityOfTimeSeries().from_returns([np.inf])
    with pytest.raises(ValueError):
        StabilityOfTimeSeries(nan_policy='raise').from_returns([np.nan])
