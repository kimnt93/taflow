import numpy as np
import pandas as pd
import pandas_ta_classic as pta

from taflow import KlingerVolumeOscillator


def test_klinger_volume_oscillator_matches_pandas_ta():
    rng = np.random.default_rng(29)
    close = 100.0 + np.cumsum(rng.normal(size=256))
    high = close + rng.uniform(0.2, 1.5, len(close))
    low = close - rng.uniform(0.2, 1.5, len(close))
    volume = rng.uniform(100.0, 1000.0, len(close))
    actual = KlingerVolumeOscillator(high, low, close, volume).compute()
    expected = pta.kvo(
        pd.Series(high), pd.Series(low), pd.Series(close), pd.Series(volume)
    )
    np.testing.assert_allclose(actual[0], expected.iloc[:, 0], equal_nan=True, atol=1e-10)
    np.testing.assert_allclose(actual[1], expected.iloc[:, 1], equal_nan=True, atol=1e-10)


def test_klinger_volume_oscillator_chunked_reset():
    close = np.linspace(90.0, 130.0, 80)
    high = close + 1.0
    low = close - 1.0
    volume = np.full_like(close, 500.0)
    whole = KlingerVolumeOscillator(high, low, close, volume, 5, 8, 3)
    chunked = KlingerVolumeOscillator(
        np.array([]), np.array([]), np.array([]), np.array([]), 5, 8, 3
    )
    chunked.extend(high[:31], low[:31], close[:31], volume[:31])
    chunked.extend(high[31:], low[31:], close[31:], volume[31:])
    for left, right in zip(whole.compute(), chunked.compute()):
        np.testing.assert_array_equal(left, right)
    assert chunked.reset() is chunked
    assert len(chunked) == 0
