import numpy as np
from taflow import VolumePriceTrend


def test_volume_price_trend_lifecycle() -> None:
    close = 100.0 + np.arange(32.0)
    volume = np.full(32, 1000.0)
    state = VolumePriceTrend(close, volume)
    first = state.compute()
    state.reset().extend(close, volume)
    np.testing.assert_array_equal(state.compute(), first)

