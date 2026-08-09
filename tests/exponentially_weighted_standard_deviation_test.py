import numpy as np
import pandas as pd

from taflow import ExponentiallyWeightedStandardDeviation


def test_matches_pandas_ewm_standard_deviation() -> None:
    values = np.linspace(1.0, 9.0, 64)
    actual = ExponentiallyWeightedStandardDeviation(values, 10).compute()
    expected = pd.Series(values).ewm(span=10, adjust=False).std(bias=True).to_numpy()
    np.testing.assert_allclose(actual, expected, equal_nan=True, rtol=1e-12, atol=1e-12)
