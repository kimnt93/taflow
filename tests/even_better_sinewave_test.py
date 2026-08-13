import numpy as np
import pandas as pd
import pandas_ta_classic as pta

from taflow import EvenBetterSinewave


def test_even_better_sinewave_matches_pandas_ta():
    close = 100.0 + np.sin(np.arange(256) / 7.0) * 3.0 + np.arange(256) * 0.02
    actual = EvenBetterSinewave(length=40).extend(close).compute()
    expected = np.asarray(pta.ebsw(pd.Series(close), length=40))
    np.testing.assert_allclose(actual, expected, equal_nan=True, atol=1e-12)


def test_even_better_sinewave_chunked_reset():
    close = np.linspace(90.0, 130.0, 100)
    whole = EvenBetterSinewave(40).extend(close)
    chunked = EvenBetterSinewave(40)
    chunked.extend(close[:31]).extend(close[31:])
    np.testing.assert_array_equal(whole.compute(), chunked.compute())
    assert chunked.reset() is chunked
    assert len(chunked) == 0
