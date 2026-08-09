import numpy as np
import pytest
import talib

from taflow import WeightedClose


def test_weighted_close_matches_talib_and_lifecycle() -> None:
    rng = np.random.default_rng(16127)
    close = 100.0 + np.cumsum(rng.normal(0.0, 0.5, 128))
    high = close + rng.uniform(0.1, 2.0, 128)
    low = close - rng.uniform(0.1, 2.0, 128)
    open = low + rng.random(128) * (high - low)
    expected = talib.WCLPRICE(high, low, close)
    actual = WeightedClose(high, low, close)
    np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=1e-12)

    state = WeightedClose([], [], [])
    assert state.extend(high[:43], low[:43], close[:43]) is state
    assert state.extend(high[43:], low[43:], close[43:]) is state
    np.testing.assert_allclose(state.compute(), expected, rtol=1e-12, atol=1e-12)
    assert state.reset() is state
    for index in range(len(close)):
        assert state.append(float(high[index]), float(low[index]), float(close[index])) is state
    np.testing.assert_allclose(state.compute(), expected, rtol=1e-12, atol=1e-12)

    fresh = WeightedClose([], [], [])
    assert len(fresh) == 0
    assert fresh.value is None
    with pytest.raises(ValueError):
        fresh.extend([1.0, 2.0], [1.0], [1.0])
    assert len(fresh) == 0
