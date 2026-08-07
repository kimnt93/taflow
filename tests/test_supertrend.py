import numpy as np
import pytest

from taflow import Supertrend


def make_ohlc(n=600, seed=3):
    rng = np.random.default_rng(seed)
    high = 100.0 + np.cumsum(rng.normal(0.0, 0.4, n))
    high += rng.random(n) * 0.5
    low = high - rng.random(n) - 0.2
    close = (high + low) / 2.0 + rng.normal(0.0, 0.1, n)
    return high, low, close


def reference_supertrend(high, low, close, length, multiplier):
    """Exact transcription of pandas-ta classic 0.6.52 `supertrend`.

    ATR is the package's RMA: true range of bar 0 is NaN (skipped by the
    pandas mean), so the seed is the mean of the first `length - 1` true
    ranges placed at bar `length - 1`, then Wilder smoothing.
    """
    n = len(close)
    true_range = np.full(n, np.nan)
    for i in range(1, n):
        true_range[i] = max(
            high[i] - low[i],
            abs(high[i] - close[i - 1]),
            abs(low[i] - close[i - 1]),
        )
    atr = np.full(n, np.nan)
    atr[length - 1] = true_range[1:length].mean()
    for i in range(length, n):
        atr[i] = (1.0 - 1.0 / length) * atr[i - 1] + (1.0 / length) * true_range[i]

    hl2 = (high + low) * 0.5
    upper = hl2 + multiplier * atr
    lower = hl2 - multiplier * atr
    direction = np.ones(n)
    trend = np.zeros(n)
    long_band = np.full(n, np.nan)
    short_band = np.full(n, np.nan)
    for i in range(1, n):
        if close[i] > upper[i - 1]:
            direction[i] = 1.0
        elif close[i] < lower[i - 1]:
            direction[i] = -1.0
        else:
            direction[i] = direction[i - 1]
            if direction[i] > 0.0 and lower[i] < lower[i - 1]:
                lower[i] = lower[i - 1]
            if direction[i] < 0.0 and upper[i] > upper[i - 1]:
                upper[i] = upper[i - 1]
        if direction[i] > 0.0:
            trend[i] = long_band[i] = lower[i]
        else:
            trend[i] = short_band[i] = upper[i]
    trend[0] = 0.0
    return trend, direction, long_band, short_band


def test_supertrend_warmup_and_reference():
    high, low, close = make_ohlc()
    length, multiplier = 7, 3.0
    st = Supertrend(high=high, low=low, close=close, timeperiod=length, multiplier=multiplier)
    trend, direction, long_band, short_band = st.compute()

    assert np.isnan(trend[: length - 1]).all()
    assert np.isnan(direction[: length - 1]).all()
    assert np.isfinite(trend[length - 1 :]).all()
    assert np.isin(direction[length - 1 :], (-1.0, 1.0)).all()

    ref = reference_supertrend(high, low, close, length, multiplier)
    assert np.allclose(trend[length - 1 :], ref[0][length - 1 :], equal_nan=True, atol=1e-12)
    assert np.array_equal(direction[length - 1 :], ref[1][length - 1 :])
    assert np.allclose(long_band[length - 1 :], ref[2][length - 1 :], equal_nan=True, atol=1e-12)
    assert np.allclose(short_band[length - 1 :], ref[3][length - 1 :], equal_nan=True, atol=1e-12)


def test_supertrend_chunk_invariance():
    high, low, close = make_ohlc()
    length, multiplier = 14, 2.5

    full = Supertrend(timeperiod=length, multiplier=multiplier)
    full.extend(high, low, close)
    trend_full, direction_full, long_full, short_full = full.compute()

    chunked = Supertrend(timeperiod=length, multiplier=multiplier)
    split = len(close) // 3
    chunked.extend(high[:split], low[:split], close[:split])
    chunked.extend(high[split:2 * split], low[split:2 * split], close[split:2 * split])
    for i in range(2 * split, len(close)):
        chunked.append(high[i], low[i], close[i])
    trend_chunked, direction_chunked, long_chunked, short_chunked = chunked.compute()

    assert np.array_equal(trend_full, trend_chunked, equal_nan=True)
    assert np.array_equal(direction_full, direction_chunked, equal_nan=True)
    assert np.array_equal(long_full, long_chunked, equal_nan=True)
    assert np.array_equal(short_full, short_chunked, equal_nan=True)


def test_supertrend_value_and_reset():
    high, low, close = make_ohlc(n=100)
    st = Supertrend(timeperiod=7, multiplier=3.0)
    assert st.value is None
    for i in range(6):
        st.append(high[i], low[i], close[i])
        assert st.value is None
    st.append(high[6], low[6], close[6])
    assert st.value is not None
    assert st.value[0] == pytest.approx(st.compute()[0][6])

    st.reset()
    assert st.value is None
    for i in range(7):
        st.append(high[i], low[i], close[i])
    assert st.value is not None
    assert st.value[0] == pytest.approx(st.compute()[0][6])


def test_supertrend_direction_flip_tracking():
    high, low, close = make_ohlc(seed=11)
    st = Supertrend(high=high, low=low, close=close, timeperiod=5, multiplier=2.0)
    _, direction, _, _ = st.compute()
    flips = np.count_nonzero(np.diff(direction[5:]) != 0)
    assert flips >= 2
    assert set(direction[5:]) <= {-1.0, 1.0}


def test_supertrend_rejects_bad_inputs():
    high, low, close = make_ohlc(n=50)
    st = Supertrend()
    with pytest.raises(ValueError):
        st.extend(high[:-1], low, close)
    with pytest.raises(ValueError):
        st.extend(high, low, close[:-1])
    with pytest.raises(ValueError):
        Supertrend(timeperiod=0)
    with pytest.raises(ValueError):
        Supertrend(multiplier=0.0)
