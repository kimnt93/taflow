import numpy as np
import pytest
import talib
from taflow import TrueRange


def test_true_range_matches_talib_and_lifecycle() -> None:
    rng = np.random.default_rng(77101)
    close = 100 + rng.normal(size=257).cumsum()
    high, low = close + rng.uniform(0.1, 2, 257), close - rng.uniform(0.1, 2, 257)
    expected = talib.TRANGE(high, low, close)
    actual = TrueRange().extend(high, low, close)
    np.testing.assert_array_equal(actual.compute(), expected)
    state = TrueRange()
    state.extend(high[:31], low[:31], close[:31]).extend(
        high[31:], low[31:], close[31:]
    )
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for i in range(len(close)):
        assert state.append(float(high[i]), float(low[i]), float(close[i])) is state
    np.testing.assert_array_equal(state.compute(), expected)
    fresh = TrueRange()
    with pytest.raises(ValueError):
        fresh.extend([1, 2], [1], [1, 2])
    assert len(fresh) == 0
