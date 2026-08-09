import numpy as np
import pandas as pd

from taflow import ExponentiallyWeightedVariance


def test_matches_pandas_ewm_variance() -> None:
    values = 100.0 + np.sin(np.arange(64) * 0.2)
    actual = ExponentiallyWeightedVariance(values, 10).compute()
    expected = pd.Series(values).ewm(span=10, adjust=False).var(bias=True).to_numpy()
    np.testing.assert_allclose(actual, expected, equal_nan=True, rtol=1e-12, atol=1e-12)


def test_lifecycle() -> None:
    state = ExponentiallyWeightedVariance(np.array([], dtype=float), 5)
    assert len(state) == 0
    assert state.append(1.0).reset().value is None
