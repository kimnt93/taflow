import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics import LongestLosingStreak

@pytest.mark.parametrize('values', [np.array([-0.1, -0.2, 0, 0.1, -0.3]), np.array([0.1, 0.2]), np.array([np.nan, -0.1, -0.2, -0.3])])
def test_matches_quantstats(values):
    usable = values[~np.isnan(values)]
    series = pd.Series(usable, index=pd.date_range('2000-01-01', periods=len(usable), freq='D'))
    assert LongestLosingStreak().from_returns(values).compute() == int(qs.consecutive_losses(series))

def test_domains_lifecycle_validation():
    values = np.array([-0.1, -0.2, 0, 0.2, -0.3])
    expected = 2
    assert LongestLosingStreak().from_pnl(values).compute() == expected
    assert LongestLosingStreak().from_trades(values).compute() == expected
    metric = LongestLosingStreak().from_returns([]).append(values[0]).extend(values[1:])
    assert metric.compute() == expected
    assert metric.reset().extend(values).compute() == expected
    unbound = LongestLosingStreak()
    with pytest.raises(ValueError):
        unbound.append(0.01)
    with pytest.raises(ValueError):
        LongestLosingStreak().from_returns([np.inf])
