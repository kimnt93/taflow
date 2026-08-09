import numpy as np
import pytest
import talib

from taflow import AccumulationDistributionOscillator


def _inputs(length: int, seed: int) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
    rng = np.random.default_rng(seed)
    base = 100.0 + np.cumsum(rng.normal(0.0, 0.7, length))
    high = base + rng.uniform(0.1, 2.0, length)
    low = base - rng.uniform(0.1, 2.0, length)
    close = low + rng.random(length) * (high - low)
    volume = rng.integers(1, 100_000, length).astype(np.float64)
    return high, low, close, volume


@pytest.mark.parametrize("fastperiod,slowperiod", [(2, 3), (3, 10), (12, 5), (5, 20)])
def test_accumulation_distribution_oscillator_matches_talib_and_lifecycle(
    fastperiod: int, slowperiod: int
) -> None:
    high, low, close, volume = _inputs(257, fastperiod * 100 + slowperiod)
    expected = talib.ADOSC(high, low, close, volume, fastperiod, slowperiod)
    actual = AccumulationDistributionOscillator(
        high, low, close, volume, fastperiod, slowperiod
    )
    np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=2e-8, equal_nan=True)

    state = AccumulationDistributionOscillator([], [], [], [], fastperiod, slowperiod)
    assert state.value is None
    assert state.extend(high[:41], low[:41], close[:41], volume[:41]) is state
    assert state.extend(high[41:], low[41:], close[41:], volume[41:]) is state
    np.testing.assert_array_equal(state.compute(), actual.compute())
    assert state.reset() is state
    for index in range(len(close)):
        assert state.append(
            float(high[index]), float(low[index]), float(close[index]), float(volume[index])
        ) is state
    np.testing.assert_array_equal(state.compute(), actual.compute())


def test_accumulation_distribution_oscillator_validates_before_mutation() -> None:
    with pytest.raises(ValueError):
        AccumulationDistributionOscillator([], [], [], [], 1, 10)
    state = AccumulationDistributionOscillator([], [], [], [])
    with pytest.raises(ValueError):
        state.extend([2.0, 3.0], [1.0], [1.5, 2.5], [10.0, 20.0])
    assert len(state) == 0
    assert state.value is None
