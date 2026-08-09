import numpy as np
import pytest
import talib

from taflow import AccumulationDistribution


def _assert_lifecycle(high: np.ndarray, low: np.ndarray, close: np.ndarray, volume: np.ndarray) -> None:
    expected = talib.AD(high, low, close, volume)
    actual = AccumulationDistribution(high, low, close, volume)
    np.testing.assert_array_equal(actual.compute(), expected)
    assert actual.value == expected[-1]

    state = AccumulationDistribution([], [], [], [])
    split = max(1, len(close) // 3)
    assert state.extend(high[:split], low[:split], close[:split], volume[:split]) is state
    assert state.extend(high[split:], low[split:], close[split:], volume[split:]) is state
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for index in range(len(close)):
        assert state.append(
            float(high[index]), float(low[index]), float(close[index]), float(volume[index])
        ) is state
    np.testing.assert_array_equal(state.compute(), expected)


def test_accumulation_distribution_matches_talib_matrix_and_lifecycle() -> None:
    rng = np.random.default_rng(33809)
    for length in [1, 64, 257]:
        close = 100.0 + np.cumsum(rng.normal(0.0, 0.7, length))
        high = close + rng.uniform(0.1, 2.0, length)
        low = close - rng.uniform(0.1, 2.0, length)
        close = low + rng.random(length) * (high - low)
        volume = rng.integers(1, 100_000, length).astype(np.float64)
        _assert_lifecycle(high, low, close, volume)

    # Constant and invalid ranges exercise TA-Lib's zero-contribution branch.
    _assert_lifecycle(
        np.array([5.0, 4.0]),
        np.array([5.0, 6.0]),
        np.array([5.0, 5.0]),
        np.array([100.0, 200.0]),
    )


def test_accumulation_distribution_rejects_misalignment_before_mutation() -> None:
    state = AccumulationDistribution([], [], [], [])
    with pytest.raises(ValueError):
        state.extend([2.0, 3.0], [1.0], [1.5, 2.5], [10.0, 20.0])
    assert len(state) == 0
    assert state.value is None
