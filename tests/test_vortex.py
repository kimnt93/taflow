import numpy as np
import pytest

from taflow import Vortex


def make_ohlc(n=600, seed=3):
    rng = np.random.default_rng(seed)
    high = 100.0 + np.cumsum(rng.normal(0.0, 0.4, n))
    high += rng.random(n) * 0.5
    low = high - rng.random(n) - 0.2
    close = (high + low) / 2.0 + rng.normal(0.0, 0.1, n)
    return high, low, close


def reference_vortex(high, low, close, n):
    """Exact transcription of bukosabino `ta` `trend.VortexIndicator`."""
    mean = close.mean()
    cs = np.concatenate(([mean], close[:-1]))
    tr = np.maximum(high - low, np.maximum(np.abs(high - cs), np.abs(low - cs)))
    vmp = np.abs(high - np.concatenate(([np.nan], low[:-1])))
    vmm = np.abs(low - np.concatenate(([np.nan], high[:-1])))

    def rolling_sum(x, win):
        out = np.full(len(x), np.nan)
        for i in range(win - 1, len(x)):
            window = x[i + 1 - win : i + 1]
            valid = window[~np.isnan(window)]
            if len(valid) >= win:
                out[i] = valid.sum()
        return out

    trn = rolling_sum(tr, n)
    vip = rolling_sum(vmp, n) / trn
    vin = rolling_sum(vmm, n) / trn
    return vip, vin


def test_vortex_warmup_and_reference():
    high, low, close = make_ohlc()
    vx = Vortex(high=high, low=low, close=close)
    vp, vn = vx.compute()

    assert np.isnan(vp[:14]).all()
    assert np.isnan(vn[:14]).all()
    assert np.isfinite(vp[14:]).all()
    assert np.isfinite(vn[14:]).all()
    assert (vp[14:] >= 0.0).all()
    assert (vn[14:] >= 0.0).all()

    ref = reference_vortex(high, low, close, 14)
    assert np.array_equal(vp, ref[0], equal_nan=True)
    assert np.array_equal(vn, ref[1], equal_nan=True)


def test_vortex_custom_window():
    high, low, close = make_ohlc(seed=7)
    vx = Vortex(high=high, low=low, close=close, window=21)
    vp, vn = vx.compute()
    ref = reference_vortex(high, low, close, 21)
    assert np.array_equal(vp, ref[0], equal_nan=True)
    assert np.array_equal(vn, ref[1], equal_nan=True)
    assert np.isnan(vp[:21]).all()
    assert np.isfinite(vp[21:]).all()


def test_vortex_chunk_invariance():
    high, low, close = make_ohlc(seed=11)
    full = Vortex(high=high, low=low, close=close)
    ref = full.compute()

    chunked = Vortex()
    split = len(close) // 4
    for start in range(0, len(close), split):
        chunked.extend(high[start:start + split], low[start:start + split], close[start:start + split])
    for got, want in zip(chunked.compute(), ref):
        assert np.array_equal(got, want, equal_nan=True)


def test_vortex_value_and_reset():
    high, low, close = make_ohlc(n=120)
    vx = Vortex()
    assert vx.value is None
    for i in range(40):
        vx.append(high[i], low[i], close[i])
    assert vx.value is not None
    assert vx.value[0] == pytest.approx(vx.compute()[0][39])
    vx.reset()
    assert vx.value is None


def test_vortex_rejects_bad_inputs():
    high, low, close = make_ohlc(n=100)
    vx = Vortex()
    with pytest.raises(ValueError):
        vx.extend(high[:-1], low, close)
    with pytest.raises(ValueError):
        vx.extend(high, low, close[:-1])
    with pytest.raises(ValueError):
        Vortex(window=0)
