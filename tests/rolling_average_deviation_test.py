import numpy as np
import pandas as pd

from taflow import RollingAverageDeviation


def test_matches_pandas_mean_absolute_deviation() -> None:
    values = np.sin(np.arange(128, dtype=np.float64) * 0.17)
    series = pd.Series(values)
    expected = series.rolling(10).apply(lambda window: np.mean(np.abs(window - np.mean(window))), raw=True).to_numpy()
    np.testing.assert_allclose(RollingAverageDeviation(10).extend(values).compute(), expected, equal_nan=True)
