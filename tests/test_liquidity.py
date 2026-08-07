import numpy as np
import pytest

from taflow import Liquidity


def test_liquidity_clusters_swings_and_detects_sweeps():
    n = 400
    high = np.full(n, 10.0)
    low = np.full(n, 9.0)
    high[60:80] = 11.0
    low[60:80] = 10.0
    high[120:140] = 11.05
    low[120:140] = 10.05
    high[160:170] = 12.0
    low[160:170] = 11.0
    high[300:320] = 12.0
    low[300:320] = 11.0
    liquidity, level, swept = Liquidity(swing_length=5, range_percent=0.02).extend(high, low).compute()
    assert np.isnan(liquidity).any()
    assert np.isin(liquidity, [-1.0, 1.0]).any()
    for i in range(n):
        if liquidity[i] == 1.0 or liquidity[i] == -1.0:
            assert level[i] > 0
    assert np.isin(swept, [-1.0, 1.0]).any() or np.isnan(swept).all()


def test_liquidity_rejects_bad_parameters_and_resets():
    with pytest.raises(ValueError):
        Liquidity(swing_length=0)
    with pytest.raises(ValueError):
        Liquidity(range_percent=-0.1)
    with pytest.raises(ValueError):
        Liquidity(range_percent=1.5)
    state = Liquidity()
    with pytest.raises(ValueError):
        state.extend(np.ones(2), np.ones(1))
    state.reset()
    assert all(len(values) == 0 for values in state.compute())
