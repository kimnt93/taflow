import numpy as np
import talib

from taflow import HilbertTransformTrendline


def test_matches_talib_ht_trendline() -> None:
    values = 100.0 + np.sin(np.arange(256) * 0.11).cumsum()
    expected = talib.HT_TRENDLINE(values)
    actual = HilbertTransformTrendline(values).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-12, equal_nan=True)
