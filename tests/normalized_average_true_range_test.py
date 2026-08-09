import numpy as np
import pytest
import talib
from taflow import NormalizedAverageTrueRange


@pytest.mark.parametrize("timeperiod", [1, 5, 14, 30])
def test_normalized_average_true_range_matches_talib_and_lifecycle(
    timeperiod: int,
) -> None:
    rng = np.random.default_rng(99317 + timeperiod)
    close = 100 + rng.normal(size=257).cumsum()
    high, low = close + rng.uniform(0.1, 2, 257), close - rng.uniform(0.1, 2, 257)
    expected = talib.NATR(high, low, close, timeperiod)
    actual = NormalizedAverageTrueRange(high, low, close, timeperiod)
    np.testing.assert_array_equal(actual.compute(), expected)
    state = NormalizedAverageTrueRange([], [], [], timeperiod)
    state.extend(high[:41], low[:41], close[:41]).extend(
        high[41:], low[41:], close[41:]
    )
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for i in range(len(close)):
        state.append(float(high[i]), float(low[i]), float(close[i]))
    np.testing.assert_array_equal(state.compute(), expected)


def test_normalized_average_true_range_validates() -> None:
    with pytest.raises(ValueError):
        NormalizedAverageTrueRange([], [], [], 0)
