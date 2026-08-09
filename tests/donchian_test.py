import numpy as np

from taflow import Donchian


def test_donchian_warmup_and_reset() -> None:
    indicator = Donchian(np.array([10.0, 12.0, 11.0]), np.array([8.0, 9.0, 7.0]), 3)
    upper, lower, middle = indicator.compute()
    np.testing.assert_allclose(upper, [np.nan, np.nan, 12.0], equal_nan=True)
    np.testing.assert_allclose(lower, [np.nan, np.nan, 7.0], equal_nan=True)
    np.testing.assert_allclose(middle, [np.nan, np.nan, 9.5], equal_nan=True)
    assert len(indicator) == 3
    assert indicator.reset().value is None
