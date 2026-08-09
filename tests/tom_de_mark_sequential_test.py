import numpy as np
import pandas as pd
import pandas_ta_classic as pta

from taflow import TomDeMarkSequential


def test_tom_de_mark_sequential_matches_pandas_ta():
    close = pd.Series(100.0 + np.sin(np.arange(256) / 3.0) * 4.0 + np.arange(256) * 0.05)
    actual_buy, actual_sell = TomDeMarkSequential(close.to_numpy()).compute()
    expected = pta.td_seq(close, asint=True)
    np.testing.assert_array_equal(actual_buy, np.minimum(expected.iloc[:, 1], 9))
    np.testing.assert_array_equal(actual_sell, np.minimum(expected.iloc[:, 0], 9))


def test_tom_de_mark_sequential_chunked_reset():
    close = np.linspace(90.0, 130.0, 40)
    whole = TomDeMarkSequential(close)
    chunked = TomDeMarkSequential(np.array([]))
    chunked.extend(close[:13]).extend(close[13:])
    for left, right in zip(whole.compute(), chunked.compute()):
        np.testing.assert_array_equal(left, right)
    assert chunked.reset() is chunked
    assert len(chunked) == 0
