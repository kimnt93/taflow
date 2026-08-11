import numpy as np
import pytest
from taflow.metrics import MaximumDrawdownDuration


def performanceanalytics_duration(returns: np.ndarray) -> int | None:
    usable = returns[~np.isnan(returns)]
    wealth = np.cumprod(1.0 + usable)
    drawdowns = wealth / np.maximum.accumulate(np.r_[1.0, wealth])[1:] - 1.0
    current = maximum = 0
    for drawdown in drawdowns:
        if drawdown < 0:
            current += 1; maximum = max(maximum, current + 1)
        elif current:
            maximum = max(maximum, current + 1); current = 0
    return maximum or None


@pytest.mark.parametrize("returns", [np.array([0.1, -0.1, 0.2]), np.array([-0.1]), np.array([0.1, -0.2, -0.1, 0.5, -0.3]), np.array([0.1, np.nan, -0.2])])
def test_matches_pinned_performanceanalytics_source(returns):
    assert MaximumDrawdownDuration.from_returns(returns).compute() == performanceanalytics_duration(returns)


def test_factories_and_lifecycle():
    returns = np.array([0.1, -0.2, 0.05, -0.1])
    equity = 100 * np.cumprod(np.r_[1.0, 1.0 + returns])
    expected = MaximumDrawdownDuration.from_returns(returns).compute()
    assert MaximumDrawdownDuration.from_equity(equity).compute() == expected
    assert MaximumDrawdownDuration.from_log_returns(np.log1p(returns)).compute() == expected
    assert MaximumDrawdownDuration.from_pnl(np.diff(equity), initial_equity=100).compute() == expected
    metric = MaximumDrawdownDuration.from_returns([]).append(returns[0]).extend(returns[1:])
    assert metric.compute() == expected and len(metric) == len(returns)
    assert metric.reset().extend(returns).compute() == expected


def test_validation():
    with pytest.raises(TypeError): MaximumDrawdownDuration()
    assert MaximumDrawdownDuration.from_returns([0.1, 0.2]).compute() is None
    with pytest.raises(ValueError): MaximumDrawdownDuration.from_returns([np.inf])
