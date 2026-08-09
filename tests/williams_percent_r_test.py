import numpy as np
import pytest
import talib

from taflow import WilliamsPercentR


@pytest.mark.parametrize("timeperiod", [2, 5, 14, 30])
def test_williams_percent_r_matches_talib_matrix_and_lifecycle(timeperiod: int) -> None:
    rng = np.random.default_rng(44021 + timeperiod)
    length = 257
    base = 100.0 + np.cumsum(rng.normal(0.0, 0.7, length))
    high = base + rng.uniform(0.1, 2.0, length)
    low = base - rng.uniform(0.1, 2.0, length)
    close = low + rng.random(length) * (high - low)
    expected = talib.WILLR(high, low, close, timeperiod)
    actual = WilliamsPercentR(high, low, close, timeperiod)
    np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True)

    state = WilliamsPercentR([], [], [], timeperiod)
    assert state.value is None
    assert state.extend(high[:43], low[:43], close[:43]) is state
    assert state.extend(high[43:], low[43:], close[43:]) is state
    np.testing.assert_array_equal(state.compute(), actual.compute())
    assert state.reset() is state
    for index in range(length):
        assert state.append(float(high[index]), float(low[index]), float(close[index])) is state
    np.testing.assert_array_equal(state.compute(), actual.compute())


def test_williams_percent_r_handles_flat_short_and_invalid_inputs() -> None:
    flat = np.full(8, 5.0)
    expected = talib.WILLR(flat, flat, flat, 4)
    np.testing.assert_array_equal(WilliamsPercentR(flat, flat, flat, 4).compute(), expected)
    short = WilliamsPercentR(flat[:2], flat[:2], flat[:2], 4)
    assert np.isnan(short.compute()).all()
    assert short.value is None
    with pytest.raises(ValueError):
        WilliamsPercentR([], [], [], 1)
    state = WilliamsPercentR([], [], [], 4)
    with pytest.raises(ValueError):
        state.extend([2.0, 3.0], [1.0], [1.5, 2.5])
    assert len(state) == 0
