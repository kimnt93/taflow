import numpy as np

from taflow import SignalDelay


def test_delays_values_and_resets() -> None:
    indicator = SignalDelay(np.arange(4.0), 2)
    np.testing.assert_allclose(indicator.compute(), [np.nan, np.nan, 0.0, 1.0], equal_nan=True)
    indicator.reset().append(8.0)
    assert indicator.value is None
