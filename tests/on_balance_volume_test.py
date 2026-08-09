import numpy as np
import pytest
import talib

from taflow import OnBalanceVolume


def _assert_lifecycle(close: np.ndarray, volume: np.ndarray) -> None:
    expected = talib.OBV(close, volume)
    actual = OnBalanceVolume(close, volume)
    np.testing.assert_array_equal(actual.compute(), expected)
    assert actual.value == expected[-1]

    chunked = OnBalanceVolume([], [])
    split = max(1, len(close) // 3)
    assert chunked.extend(close[:split], volume[:split]) is chunked
    assert chunked.extend(close[split:], volume[split:]) is chunked
    np.testing.assert_array_equal(chunked.compute(), expected)
    assert chunked.reset() is chunked
    assert chunked.value is None
    assert len(chunked) == 0
    for index in range(len(close)):
        assert chunked.append(float(close[index]), float(volume[index])) is chunked
    np.testing.assert_array_equal(chunked.compute(), expected)


def test_on_balance_volume_matches_talib_parameter_matrix_and_lifecycle() -> None:
    rng = np.random.default_rng(18841)
    random_close = 100.0 + np.cumsum(rng.normal(0.0, 0.7, 257))
    cases = [
        np.array([4.0], dtype=np.float64),
        np.full(64, 12.0, dtype=np.float64),
        np.arange(1.0, 66.0, dtype=np.float64),
        np.arange(65.0, 0.0, -1.0, dtype=np.float64),
        np.repeat(np.array([5.0, 6.0, 6.0, 4.0]), 32),
        random_close,
    ]
    for close in cases:
        volume = rng.integers(1, 100_000, len(close)).astype(np.float64)
        _assert_lifecycle(close, volume)


def test_on_balance_volume_rejects_misaligned_input_before_mutation() -> None:
    state = OnBalanceVolume([], [])
    with pytest.raises(ValueError):
        state.extend([1.0, 2.0], [3.0])
    assert len(state) == 0
    assert state.value is None
