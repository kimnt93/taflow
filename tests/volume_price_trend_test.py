import numpy as np
from taflow import VolumePriceTrend
from tests.oracle_assertions import assert_registered_oracle_match


def test_volume_price_trend_matches_registered_wickra_oracle() -> None:
    assert_registered_oracle_match("VolumePriceTrend")


def test_volume_price_trend_lifecycle() -> None:
    close = 100.0 + np.arange(32.0)
    volume = np.full(32, 1000.0)
    state = VolumePriceTrend(close, volume)
    first = state.compute()
    state.reset().extend(close, volume)
    np.testing.assert_array_equal(state.compute(), first)
