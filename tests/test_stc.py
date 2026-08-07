import numpy as np
import pytest

from taflow import Stc


def make_close(n=600, seed=3):
    rng = np.random.default_rng(seed)
    return 100.0 + np.cumsum(rng.normal(0.0, 0.4, n)) + rng.normal(0.0, 0.1, n)


def non_zero_range(high, low):
    diff = high - low
    if np.any(diff == 0):
        diff = diff + np.finfo(np.float64).eps
    return diff


def reference_stc(close, tclength, fast, slow, factor):
    """Exact transcription of pandas-ta classic `momentum/stc.py`."""
    if slow < fast:
        fast, slow = slow, fast
    n = len(close)

    def ema(x, length):
        out = np.full(n, np.nan)
        out[length - 1] = x[:length].mean()
        alpha = 2.0 / (length + 1)
        for i in range(length, n):
            out[i] = out[i - 1] + alpha * (x[i] - out[i - 1])
        return out

    xmacd = ema(close, fast) - ema(close, slow)

    def rolling_extrema(x, win):
        lo = np.full(n, np.nan)
        hi = np.full(n, np.nan)
        for i in range(win - 1, n):
            window = x[i + 1 - win : i + 1]
            if np.isnan(window).any():
                continue
            lo[i] = window.min()
            hi[i] = window.max()
        return lo, hi

    lowest, highest = rolling_extrema(xmacd, tclength)
    xrange = non_zero_range(highest, lowest)

    stoch1 = list(xmacd.copy())
    pf = list(xmacd.copy())
    stoch1[0], pf[0] = 0, 0
    for i in range(1, n):
        if lowest[i] > 0:
            stoch1[i] = 100 * ((xmacd[i] - lowest[i]) / xrange[i])
        else:
            stoch1[i] = stoch1[i - 1]
        pf[i] = round(pf[i - 1] + factor * (stoch1[i] - pf[i - 1]), 8)
    pf = np.array(pf)

    lowest_pf, highest_pf = rolling_extrema(pf, tclength)
    pfrange = non_zero_range(highest_pf, lowest_pf)

    stoch2 = list(xmacd.copy())
    pff = list(xmacd.copy())
    stoch2[0], pff[0] = 0, 0
    for i in range(1, n):
        if pfrange[i] > 0:
            stoch2[i] = 100 * ((pf[i] - lowest_pf[i]) / pfrange[i])
        else:
            stoch2[i] = stoch2[i - 1]
        pff[i] = round(pff[i - 1] + factor * (stoch2[i] - pff[i - 1]), 8)
    return np.array(pff), xmacd, pf


def test_stc_warmup_and_reference():
    close = make_close()
    st = Stc(close=close)
    stc, macd, stoch = st.compute()

    assert stc[0] == 0.0
    assert stoch[0] == 0.0
    assert np.isnan(macd[:24]).all()
    assert np.isfinite(macd[25:]).all()
    assert np.isfinite(stc).all()
    assert np.isfinite(stoch).all()
    assert (stc >= 0.0).all() and (stc <= 100.0).all()
    assert (stoch >= 0.0).all() and (stoch <= 100.0).all()

    # pandas-ta `non_zero_range` adds f64 epsilon to the *whole* series when any
    # element is exactly zero (common in the 8-decimal-rounded pf series). That
    # global perturbation is not reproducible in a streaming state and only
    # shifts outputs by ~1e-8 when a value sits on a rounding boundary, so the
    # parity check uses a small tolerance.
    ref = reference_stc(close, 10, 12, 26, 0.5)
    assert np.allclose(stc, ref[0], equal_nan=True, atol=1e-5)
    assert np.allclose(stoch, ref[2], equal_nan=True, atol=1e-5)
    assert np.allclose(macd, ref[1], equal_nan=True, atol=1e-12)


def test_stc_custom_params():
    close = make_close(seed=7)
    st = Stc(close=close, tclength=14, fast=5, slow=21, factor=0.7)
    stc, macd, stoch = st.compute()
    ref = reference_stc(close, 14, 5, 21, 0.7)
    assert np.allclose(stc, ref[0], equal_nan=True, atol=1e-5)
    assert np.allclose(stoch, ref[2], equal_nan=True, atol=1e-5)
    assert np.allclose(macd, ref[1], equal_nan=True, atol=1e-12)


def test_stc_fast_slow_swap():
    close = make_close(seed=11)
    a = Stc(close=close, fast=12, slow=26).compute()[0]
    b = Stc(close=close, fast=26, slow=12).compute()[0]
    assert np.array_equal(a, b)


def test_stc_chunk_invariance():
    close = make_close(seed=13)
    full = Stc(close=close)
    ref = full.compute()

    chunked = Stc()
    split = len(close) // 4
    for start in range(0, len(close), split):
        chunked.extend(close[start:start + split])
    for got, want in zip(chunked.compute(), ref):
        assert np.array_equal(got, want, equal_nan=True)


def test_stc_value_and_reset():
    close = make_close(n=120)
    st = Stc()
    assert st.value is None
    for i in range(60):
        st.append(close[i])
    assert st.value is not None
    assert st.value[0] == pytest.approx(st.compute()[0][59])
    st.reset()
    assert st.value is None


def test_stc_rejects_bad_inputs():
    with pytest.raises(ValueError):
        Stc(tclength=0)
    with pytest.raises(ValueError):
        Stc(fast=0)
    with pytest.raises(ValueError):
        Stc(slow=0)
    with pytest.raises(ValueError):
        Stc(factor=0.0)
