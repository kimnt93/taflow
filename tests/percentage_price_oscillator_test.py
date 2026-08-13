import numpy as np
import talib

from taflow import PercentagePriceOscillator


def test_matches_talib_ppo() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    expected = talib.PPO(values, 12, 26, 0)
    actual = PercentagePriceOscillator().extend(values).compute()
    np.testing.assert_allclose(actual, expected, equal_nan=True)
