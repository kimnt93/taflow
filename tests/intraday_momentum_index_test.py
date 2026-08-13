import numpy as np
from taflow import IntradayMomentumIndex


def test_intraday_momentum_index_lifecycle():
    open = np.linspace(90.0, 120.0, 64); close = open + np.sin(np.arange(64))
    indicator = IntradayMomentumIndex(14).extend(open, close); first = indicator.compute()
    indicator.reset().extend(open, close)
    np.testing.assert_array_equal(first, indicator.compute())
