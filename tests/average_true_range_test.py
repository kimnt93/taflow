import numpy as np
import pytest
import talib
from taflow import AverageTrueRange


@pytest.mark.parametrize("timeperiod", [1, 5, 14, 30])
def test_average_true_range_matches_talib_and_lifecycle(timeperiod: int) -> None:
    rng = np.random.default_rng(88211 + timeperiod)
    close = 100 + rng.normal(size=257).cumsum()
    high, low = close + rng.uniform(0.1, 2, 257), close - rng.uniform(0.1, 2, 257)
    expected = talib.ATR(high, low, close, timeperiod)
    actual = AverageTrueRange(timeperiod).extend(high, low, close)
    np.testing.assert_array_equal(actual.compute(), expected)
    state = AverageTrueRange(timeperiod)
    state.extend(high[:37], low[:37], close[:37]).extend(
        high[37:], low[37:], close[37:]
    )
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for i in range(len(close)):
        state.append(float(high[i]), float(low[i]), float(close[i]))
    np.testing.assert_array_equal(state.compute(), expected)


def test_average_true_range_validates() -> None:
    with pytest.raises(ValueError):
        AverageTrueRange(0)
    state = AverageTrueRange()
    with pytest.raises(ValueError):
        state.extend([1, 2], [1], [1, 2])
    assert len(state) == 0
