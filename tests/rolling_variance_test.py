import numpy as np
import pandas as pd

from taflow import RollingVariance


def test_matches_pandas_population_variance() -> None:
    values = np.sin(np.arange(128, dtype=np.float64) * 0.17)
    expected = pd.Series(values).rolling(10).var(ddof=0).to_numpy()
    np.testing.assert_allclose(RollingVariance(values, 10).compute(), expected, equal_nan=True)
