import numpy as np
import pytest
from taflow.metrics import BreakevenRate

@pytest.mark.parametrize('values', [np.array([0.0, 1.0, -1.0, -0.0]), np.array([np.nan, 0.0, 2.0]), np.zeros(20)])
def test_matches_numpy(values):
    usable = values[~np.isnan(values)]
    expected = float(np.count_nonzero(usable == 0.0) / len(usable))
    assert BreakevenRate().from_returns(values).compute() == pytest.approx(expected)

def test_domains_and_lifecycle():
    values = np.array([0.0, 0.3, -0.2, 0.0])
    expected = BreakevenRate().from_returns(values).compute()
    assert BreakevenRate().from_pnl(values).compute() == expected
    assert BreakevenRate().from_trades(values).compute() == expected
    metric = BreakevenRate().from_trades([]).append(0).extend(values[1:])
    assert metric.compute() == expected and len(metric) == 4
    assert metric.reset().extend(values).compute() == expected

def test_validation_and_warmup():
    unbound = BreakevenRate()
    with pytest.raises(ValueError):
        unbound.append(0.01)
    assert BreakevenRate().from_returns([]).compute() is None
    with pytest.raises(ValueError):
        BreakevenRate().from_returns([np.inf])
    with pytest.raises(ValueError):
        BreakevenRate(nan_policy='raise').from_returns([np.nan])
