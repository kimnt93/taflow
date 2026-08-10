import numpy as np

from taflow import KeltnerChannels
from tests.oracle_assertions import assert_registered_oracle_match


def test_keltner_outputs_are_aligned_and_resettable() -> None:
    indicator = KeltnerChannels(
        np.array([12.0, 13.0, 14.0]),
        np.array([8.0, 9.0, 10.0]),
        np.array([10.0, 11.0, 12.0]),
        2,
        2.0,
    )
    upper, middle, lower = indicator.compute()
    np.testing.assert_allclose(middle, [np.nan, 10.5, 11.5], equal_nan=True)
    np.testing.assert_allclose(upper - middle, [np.nan, 8.0, 8.0], equal_nan=True)
    np.testing.assert_allclose(middle - lower, upper - middle)
    assert len(indicator) == 3
    assert indicator.reset().value is None


def test_keltner_channels_match_wickra() -> None:
    assert_registered_oracle_match("KeltnerChannels")
