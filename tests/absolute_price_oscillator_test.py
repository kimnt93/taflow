import numpy as np
import talib

from taflow import AbsolutePriceOscillator


def test_matches_talib_apo() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    expected = talib.APO(values, 12, 26, 0)
    actual = AbsolutePriceOscillator().extend(values).compute()
    np.testing.assert_allclose(actual, expected, equal_nan=True)
