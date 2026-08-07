import numpy as np
import pytest

from taflow import Retracements


def test_retracements_tracks_legs_and_deepest_pct():
    n = 300
    high = np.full(n, 100.0)
    low = np.full(n, 90.0)
    close = np.full(n, 95.0)
    high[30:50] = np.linspace(100.0, 110.0, 20)
    low[30:50] = np.linspace(90.0, 100.0, 20)
    close[30:50] = np.linspace(95.0, 105.0, 20)
    high[60:80] = np.linspace(110.0, 95.0, 20)
    low[60:80] = np.linspace(100.0, 85.0, 20)
    close[60:80] = np.linspace(105.0, 90.0, 20)
    direction, current, deepest = Retracements(swing_length=3).extend(high, low, close).compute()
    assert np.isnan(direction).any()
    assert np.isin(direction, [-1.0, 1.0]).any()
    for i in range(n):
        if not np.isnan(current[i]):
            assert 0.0 <= current[i] <= 100.0
            assert deepest[i] >= current[i]


def test_retracements_rejects_bad_parameters_and_resets():
    with pytest.raises(ValueError):
        Retracements(swing_length=0)
    state = Retracements()
    with pytest.raises(ValueError):
        state.extend(np.ones(2), np.ones(2), np.ones(1))
    state.reset()
    assert all(len(values) == 0 for values in state.compute())
