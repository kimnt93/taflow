import numpy as np
import pytest
from taflow.metrics import NetProfit

@pytest.mark.parametrize('values', [np.array([10.0, -4.0, 0.0, -1.0]), np.array([np.nan, 2.0, -3.0]), np.zeros(10)])
def test_matches_numpy(values):
    usable = values[~np.isnan(values)]
    assert NetProfit().from_pnl(values).compute() == pytest.approx(float(np.sum(usable)))

def test_domains_lifecycle_validation():
    values = np.array([3.0, -7.0, 10.0])
    expected = 6.0
    assert NetProfit().from_trades(values).compute() == expected
    metric = NetProfit().from_pnl([]).append(3).extend(values[1:])
    assert metric.compute() == expected
    assert metric.reset().extend(values).compute() == expected
    unbound = NetProfit()
    with pytest.raises(ValueError):
        unbound.append(0.01)
    with pytest.raises(ValueError):
        NetProfit().from_pnl([np.inf])
