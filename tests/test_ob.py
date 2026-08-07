import numpy as np
import pytest

from taflow import Ob


def test_ob_detects_bullish_and_bearish_blocks_with_volume():
    n = 400
    high = np.full(n, 10.0)
    low = np.full(n, 9.0)
    close = np.full(n, 9.5)
    volume = np.full(n, 100.0)
    high[100:130] = np.linspace(10.0, 14.0, 30)
    close[100:130] = np.linspace(9.5, 13.5, 30)
    low[100:130] = np.linspace(9.0, 13.0, 30)
    high[200:240] = np.linspace(14.0, 9.0, 40)
    close[200:240] = np.linspace(13.5, 8.5, 40)
    low[200:240] = np.linspace(13.0, 8.0, 40)
    high[300:350] = np.linspace(9.0, 15.0, 50)
    close[300:350] = np.linspace(8.5, 14.5, 50)
    low[300:350] = np.linspace(8.0, 14.0, 50)
    ob, top, bottom, ob_volume, mitigated = Ob().extend(high, low, close, volume).compute()
    assert ob[0] != ob[0]
    assert np.isnan(ob).any()
    assert np.nanmax(ob) == 1.0
    assert np.nanmin(ob) == -1.0
    for i in range(n):
        if ob[i] == 1.0:
            assert top[i] >= bottom[i]
            assert ob_volume[i] > 0
        if ob[i] == -1.0:
            assert top[i] >= bottom[i]
            assert ob_volume[i] > 0
    assert np.isin(mitigated, [-1.0, 1.0]).any() or np.isnan(mitigated).all()


def test_ob_rejects_bad_parameters_and_resets():
    with pytest.raises(ValueError):
        Ob(swing_length=0)
    with pytest.raises(ValueError):
        Ob(atr_period=0)
    with pytest.raises(ValueError):
        Ob(threshold=-1.0)
    state = Ob()
    with pytest.raises(ValueError):
        state.extend(np.ones(2), np.ones(2), np.ones(1), np.ones(2))
    state.reset()
    assert all(len(values) == 0 for values in state.compute())
