import numpy as np
import pytest
import talib

from taflow import AveragePrice


def test_average_price_matches_talib_and_lifecycle() -> None:
    rng = np.random.default_rng(16127)
    close = 100.0 + np.cumsum(rng.normal(0.0, 0.5, 128))
    high = close + rng.uniform(0.1, 2.0, 128)
    low = close - rng.uniform(0.1, 2.0, 128)
    open = low + rng.random(128) * (high - low)
    expected = talib.AVGPRICE(open, high, low, close)
    actual = AveragePrice(open, high, low, close)
    np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=1e-12)

    state = AveragePrice([], [], [], [])
    assert state.extend(open[:43], high[:43], low[:43], close[:43]) is state
    assert state.extend(open[43:], high[43:], low[43:], close[43:]) is state
    np.testing.assert_allclose(state.compute(), expected, rtol=1e-12, atol=1e-12)
    assert state.reset() is state
    for index in range(len(close)):
        assert state.append(float(open[index]), float(high[index]), float(low[index]), float(close[index])) is state
    np.testing.assert_allclose(state.compute(), expected, rtol=1e-12, atol=1e-12)

    fresh = AveragePrice([], [], [], [])
    assert len(fresh) == 0
    assert fresh.value is None
    with pytest.raises(ValueError):
        fresh.extend([1.0, 2.0], [1.0], [1.0], [1.0])
    assert len(fresh) == 0
