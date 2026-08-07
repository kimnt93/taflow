import numpy as np
from taflow import Vpt


def test_vpt_reference_and_chunks():
    close = np.arange(1.0, 101.0)
    volume = np.arange(1.0, 101.0)
    expected = np.full(100, np.nan)
    expected[1:] = np.cumsum(volume[1:] * np.diff(close) / close[:-1])
    full = Vpt(close=close, volume=volume).compute()
    np.testing.assert_allclose(full, expected, equal_nan=True)
    chunked = Vpt()
    for start in range(0, 100, 11): chunked.extend(close[start:start+11], volume[start:start+11])
    np.testing.assert_array_equal(chunked.compute(), full)
