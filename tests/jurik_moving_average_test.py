import numpy as np
import pandas as pd
import pandas_ta_classic as pta

from taflow import JurikMovingAverage


def test_matches_pandas_ta_classic_jma() -> None:
    values = 100.0 + np.sin(np.arange(256) * 0.13).cumsum()
    expected = np.asarray(pta.jma(pd.Series(values), length=7, phase=0.0))
    actual = JurikMovingAverage(7, 0.0).extend(values).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-10, equal_nan=True)
