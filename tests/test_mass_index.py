import numpy as np

from taflow import MassIndex


def test_mass_index_warmup_and_chunk_invariance():
    close = 100.0 + np.cumsum(np.sin(np.arange(200.0)) * 0.2)
    high = close + 1.0 + np.sin(np.arange(200.0) * 0.3) * 0.1
    low = close - 1.0
    full = MassIndex(high=high, low=low).compute()
    assert np.isnan(full[:40]).all()
    assert np.isfinite(full[40:]).all()

    chunked = MassIndex()
    for start in range(0, len(close), 17):
        chunked.extend(high[start : start + 17], low[start : start + 17])
    np.testing.assert_array_equal(chunked.compute(), full)

    alpha = 2.0 / 10.0
    ema1 = np.empty(len(high))
    ema2 = np.full(len(high), np.nan)
    ema1[0] = high[0] - low[0]
    for index in range(1, len(high)):
        ema1[index] = ema1[index - 1] + alpha * ((high[index] - low[index]) - ema1[index - 1])
    ema2[8] = ema1[8]
    for index in range(9, len(high)):
        ema2[index] = ema2[index - 1] + alpha * (ema1[index] - ema2[index - 1])
    expected = np.full(len(high), np.nan)
    ratio = ema1 / ema2
    ratio[:16] = np.nan
    for index in range(40, len(high)):
        expected[index] = ratio[index - 24 : index + 1].sum()
    np.testing.assert_allclose(full, expected, equal_nan=True, atol=1e-12)


def test_mass_index_reset():
    high = np.arange(100.0) + 2.0
    low = np.arange(100.0)
    state = MassIndex(high=high, low=low)
    expected = state.compute()
    state.reset()
    state.extend(high, low)
    np.testing.assert_array_equal(state.compute(), expected)
