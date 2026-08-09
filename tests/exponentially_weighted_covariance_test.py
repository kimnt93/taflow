import numpy as np
import pandas as pd

from taflow import ExponentiallyWeightedCovariance


def test_matches_pandas_ewm_covariance() -> None:
    left = np.linspace(1.0, 9.0, 64)
    right = np.sin(np.arange(64) * 0.3)
    actual = ExponentiallyWeightedCovariance(left, right, 10).compute()
    expected = pd.Series(left).ewm(span=10, adjust=False).cov(pd.Series(right), bias=True).to_numpy()
    np.testing.assert_allclose(actual, expected, equal_nan=True, rtol=1e-11, atol=1e-11)
