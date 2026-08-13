import numpy as np

from taflow import SignalDelay


def test_delays_values_and_resets() -> None:
    indicator = SignalDelay(2).extend(np.arange(4.0))
    np.testing.assert_allclose(indicator.compute(), [np.nan, np.nan, 0.0, 1.0], equal_nan=True)
    indicator.reset().append(8.0)
    assert indicator.value is None
