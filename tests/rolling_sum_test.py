import numpy as np
import pandas as pd

from taflow import RollingSum


def test_matches_pandas_rolling_sum() -> None:
    values = np.arange(128, dtype=np.float64) * 0.5
    expected = pd.Series(values).rolling(10).sum().to_numpy()
    np.testing.assert_allclose(RollingSum(values, 10).compute(), expected, equal_nan=True)
