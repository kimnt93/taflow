import numpy as np
import pytest
import talib

from taflow import BalanceOfPower


def _prices(close: np.ndarray, seed: int) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    rng = np.random.default_rng(seed)
    high = close + rng.uniform(0.1, 2.0, len(close))
    low = close - rng.uniform(0.1, 2.0, len(close))
    open = low + rng.random(len(close)) * (high - low)
    return open, high, low


def _assert_lifecycle(open: np.ndarray, high: np.ndarray, low: np.ndarray, close: np.ndarray) -> None:
    expected = talib.BOP(open, high, low, close)
    actual = BalanceOfPower(open, high, low, close)
    np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=1e-12)
    assert actual.value == expected[-1]

    chunked = BalanceOfPower([], [], [], [])
    split = max(1, len(close) // 3)
    assert chunked.extend(open[:split], high[:split], low[:split], close[:split]) is chunked
    assert chunked.extend(open[split:], high[split:], low[split:], close[split:]) is chunked
    np.testing.assert_allclose(chunked.compute(), expected, rtol=1e-12, atol=1e-12)
    assert chunked.reset() is chunked
    assert chunked.value is None
    assert len(chunked) == 0
    for index in range(len(close)):
        assert chunked.append(
            float(open[index]), float(high[index]), float(low[index]), float(close[index])
        ) is chunked
    np.testing.assert_allclose(chunked.compute(), expected, rtol=1e-12, atol=1e-12)


def test_balance_of_power_matches_talib_parameter_matrix_and_lifecycle() -> None:
    rng = np.random.default_rng(29107)
    cases = [
        np.array([4.0], dtype=np.float64),
        np.full(64, 12.0, dtype=np.float64),
        np.arange(1.0, 66.0, dtype=np.float64),
        np.repeat(np.array([5.0, 6.0, 6.0, 4.0]), 32),
        100.0 + np.cumsum(rng.normal(0.0, 0.7, 257)),
    ]
    for seed, close in enumerate(cases, 1):
        open, high, low = _prices(close, seed)
        _assert_lifecycle(open, high, low, close)

    # TA-Lib defines non-positive ranges as zero.
    close = np.array([2.0, 2.0], dtype=np.float64)
    _assert_lifecycle(
        np.array([1.0, 1.0]),
        np.array([1.0, 2.0]),
        np.array([2.0, 2.0]),
        close,
    )


def test_balance_of_power_rejects_misaligned_input_before_mutation() -> None:
    state = BalanceOfPower([], [], [], [])
    with pytest.raises(ValueError):
        state.extend([1.0, 2.0], [2.0], [0.0, 0.0], [1.0, 1.0])
    assert len(state) == 0
    assert state.value is None
