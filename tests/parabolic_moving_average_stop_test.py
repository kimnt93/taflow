import numpy as np
import pandas as pd
import pandas_ta_classic as pta

from taflow import ParabolicMovingAverageStop


def test_parabolic_moving_average_stop_matches_pandas_ta():
    rng = np.random.default_rng(17)
    close = 100.0 + np.cumsum(rng.normal(size=256))
    high = close + rng.uniform(0.2, 2.0, len(close))
    low = close - rng.uniform(0.2, 2.0, len(close))
    actual_stop, actual_trend = ParabolicMovingAverageStop(
        high, low, close, length=10, multiplier=3.0
    ).compute()
    expected = np.asarray(
        pta.pmax(pd.Series(high), pd.Series(low), pd.Series(close), length=10, multiplier=3.0)
    )
    np.testing.assert_allclose(actual_stop, expected, equal_nan=True, rtol=0.0, atol=1e-12)
    assert actual_trend.shape == close.shape


def test_parabolic_moving_average_stop_lifecycle_is_chunk_invariant():
    close = np.linspace(90.0, 130.0, 64)
    high = close + 1.0
    low = close - 1.0
    whole = ParabolicMovingAverageStop(high, low, close, 7, 2.0)
    chunked = ParabolicMovingAverageStop(np.array([]), np.array([]), np.array([]), 7, 2.0)
    chunked.extend(high[:20], low[:20], close[:20]).extend(high[20:], low[20:], close[20:])
    for left, right in zip(whole.compute(), chunked.compute()):
        np.testing.assert_array_equal(left, right)
    assert chunked.reset() is chunked
    assert len(chunked) == 0
