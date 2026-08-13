import numpy as np
import pandas as pd

from taflow import ExponentiallyWeightedCorrelation


def test_matches_pandas_ewm_correlation() -> None:
    left = np.linspace(1.0, 9.0, 64)
    right = np.sin(np.arange(64) * 0.3)
    actual = ExponentiallyWeightedCorrelation(10).extend(left, right).compute()
    expected = pd.Series(left).ewm(span=10, adjust=False).corr(pd.Series(right)).to_numpy().copy()
    expected[0] = 0.0
    np.testing.assert_allclose(actual, expected, equal_nan=True, rtol=1e-11, atol=1e-11)
