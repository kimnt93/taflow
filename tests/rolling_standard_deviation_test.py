import numpy as np
import pandas as pd

from taflow import RollingStandardDeviation


def test_matches_pandas_population_std() -> None:
    values = np.sin(np.arange(128, dtype=np.float64) * 0.17)
    expected = pd.Series(values).rolling(10).std(ddof=0).to_numpy()
    np.testing.assert_allclose(RollingStandardDeviation(values, 10).compute(), expected, equal_nan=True)
