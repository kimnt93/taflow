import numpy as np
import pytest

from taflow import EqualHighsLows


def test_equal_highs_lows_detects_equal_pivots():
    n = 400
    high = np.full(n, 10.0)
    low = np.full(n, 9.0)
    close = np.full(n, 9.5)
    high[50:70] = 11.0
    low[50:70] = 9.0
    high[80:100] = 11.0
    low[80:100] = 9.0
    high[150:170] = 8.0
    low[150:170] = 7.0
    high[200:220] = 8.0
    low[200:220] = 7.0
    eqh, eql, level = EqualHighsLows(eq_len=3, atr_period=50, eq_threshold=0.1).extend(high, low, close).compute()
    assert np.isin(eqh, [1.0]).any()
    assert np.isin(eql, [1.0]).any()
    for i in range(n):
        if eqh[i] == 1.0 or eql[i] == 1.0:
            assert level[i] > 0


def test_equal_highs_lows_rejects_bad_parameters_and_resets():
    with pytest.raises(ValueError):
        EqualHighsLows(eq_len=0)
    with pytest.raises(ValueError):
        EqualHighsLows(atr_period=0)
    with pytest.raises(ValueError):
        EqualHighsLows(eq_threshold=-0.1)
    state = EqualHighsLows()
    with pytest.raises(ValueError):
        state.extend(np.ones(2), np.ones(2), np.ones(1))
    state.reset()
    assert all(len(values) == 0 for values in state.compute())
