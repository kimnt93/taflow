import numpy as np
import pytest

from taflow import HedgeRatio


def test_hedge_ratio_matches_rolling_ols_and_chunks():
    x = np.arange(1.0, 9.0)
    y = 2.0 * x + 3.0
    expected = np.full(x.size, np.nan)
    expected[2:] = 2.0
    state = HedgeRatio(timeperiod=3)
    np.testing.assert_array_equal(state.extend(x, y).compute(), expected)
    chunked = HedgeRatio(timeperiod=3)
    chunked.extend(x[:4], y[:4]).extend(x[4:], y[4:])
    np.testing.assert_array_equal(chunked.compute(), expected)


def test_hedge_ratio_rejects_invalid_inputs():
    with pytest.raises(ValueError):
        HedgeRatio(timeperiod=0)
    with pytest.raises(ValueError):
        HedgeRatio().extend(np.ones(3), np.ones(2))
