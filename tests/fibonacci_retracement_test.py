import numpy as np
import pandas as pd
import pytest

from taflow import FibonacciRetracement


RATIOS = (0.0, 0.236, 0.382, 0.5, 0.618, 0.786, 1.0)


def pandas_reference(close: np.ndarray, window: int) -> tuple[np.ndarray, ...]:
    series = pd.Series(close)
    high = series.rolling(window, min_periods=1).max()
    low = series.rolling(window, min_periods=1).min()
    span = high - low
    return tuple((high - span * ratio).to_numpy() for ratio in RATIOS)


@pytest.mark.parametrize("window", [1, 2, 7, 120])
def test_fibonacci_retracement_matches_pandas(window: int) -> None:
    rng = np.random.default_rng(0xF1B0)
    random = 100.0 + rng.normal(size=257).cumsum()
    repeated = np.resize(np.array([100.0, 105.0, 105.0, 95.0]), 257)
    for close in (
        np.array([100.0]),
        np.full(257, 100.0),
        np.arange(257, dtype=np.float64),
        repeated,
        random,
    ):
        actual = FibonacciRetracement(window).extend(close).compute()
        expected = pandas_reference(close, window)
        for actual_level, expected_level in zip(actual, expected, strict=True):
            np.testing.assert_array_equal(actual_level, expected_level)


def test_fibonacci_retracement_lifecycle_is_invariant() -> None:
    rng = np.random.default_rng(8128)
    close = 100.0 + rng.normal(size=301).cumsum()
    batch = FibonacciRetracement(30).extend(close)

    chunked = FibonacciRetracement(30)
    assert chunked.extend(close[:43]) is chunked
    assert chunked.extend(close[43:211]) is chunked
    assert chunked.extend(close[211:]) is chunked
    for chunked_level, batch_level in zip(
        chunked.compute(), batch.compute(), strict=True
    ):
        np.testing.assert_array_equal(chunked_level, batch_level)

    assert chunked.reset() is chunked
    assert chunked.value is None
    for value in close:
        assert chunked.append(float(value)) is chunked
    for replay_level, batch_level in zip(
        chunked.compute(), batch.compute(), strict=True
    ):
        np.testing.assert_array_equal(replay_level, batch_level)
    assert chunked.value == batch.value
    assert len(chunked) == len(close)


def test_fibonacci_retracement_validates_configuration_and_input() -> None:
    with pytest.raises(ValueError):
        FibonacciRetracement(0)
    with pytest.raises(ValueError):
        FibonacciRetracement().extend(None)
