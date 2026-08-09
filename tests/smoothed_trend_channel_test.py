import numpy as np

from taflow import SmoothedTrendChannel


def test_warmup_and_reset() -> None:
    indicator = SmoothedTrendChannel(
        np.array([10.0, 12.0]), np.array([8.0, 9.0]), np.array([9.0, 11.0]), 2
    )
    lower, upper = indicator.compute()
    np.testing.assert_allclose(lower, [np.nan, 8.5], equal_nan=True)
    np.testing.assert_allclose(upper, [np.nan, 11.0], equal_nan=True)
    assert indicator.reset().value is None
