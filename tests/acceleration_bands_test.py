import numpy as np

from taflow import AccelerationBands


def test_acceleration_bands_lifecycle() -> None:
    close = np.linspace(100.0, 112.0, 12)
    high = close + 1.0
    low = close - 1.0
    indicator = AccelerationBands(high, low, close, period=3)
    expected = indicator.compute()
    indicator.reset().extend(high, low, close)
    for actual, want in zip(indicator.compute(), expected):
        np.testing.assert_array_equal(actual, want)
    assert len(indicator) == len(close)
