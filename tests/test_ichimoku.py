import numpy as np
import pytest

from taflow import Ichimoku


def make_ohlc(n=500, seed=3):
    rng = np.random.default_rng(seed)
    high = 100.0 + np.cumsum(rng.normal(0.0, 0.4, n))
    high += rng.random(n) * 0.5
    low = high - rng.random(n) - 0.2
    close = (high + low) / 2.0 + rng.normal(0.0, 0.1, n)
    return high, low, close


def reference_ichimoku(high, low, close, tenkan, kijun, senkou):
    """Causal transcription of pandas-ta classic `ichimoku` (no displacement).

    Each component is the rolling `(max high + min low) / 2` over its window;
    span_a = 0.5 * (tenkan_sen + kijun_sen); chikou is the current close.
    """
    n = len(close)

    def midprice(period):
        out = np.full(n, np.nan)
        for i in range(period - 1, n):
            out[i] = (
                high[i + 1 - period : i + 1].max() + low[i + 1 - period : i + 1].min()
            ) * 0.5
        return out

    tenkan_sen = midprice(tenkan)
    kijun_sen = midprice(kijun)
    span_b = midprice(senkou)
    span_a = np.where(
        np.isnan(tenkan_sen) | np.isnan(kijun_sen), np.nan, 0.5 * (tenkan_sen + kijun_sen)
    )
    return tenkan_sen, kijun_sen, span_a, span_b, close.copy()


def test_ichimoku_warmup_and_reference():
    high, low, close = make_ohlc()
    tenkan, kijun, senkou = 9, 26, 52
    ik = Ichimoku(high=high, low=low, close=close, tenkan=tenkan, kijun=kijun, senkou=senkou)
    tenkan_sen, kijun_sen, span_a, span_b, chikou = ik.compute()

    assert np.isnan(tenkan_sen[: tenkan - 1]).all()
    assert np.isnan(kijun_sen[: kijun - 1]).all()
    assert np.isnan(span_a[: kijun - 1]).all()
    assert np.isnan(span_b[: senkou - 1]).all()
    assert np.isfinite(tenkan_sen[tenkan - 1 :]).all()
    assert np.isfinite(kijun_sen[kijun - 1 :]).all()
    assert np.isfinite(span_a[kijun - 1 :]).all()
    assert np.isfinite(span_b[senkou - 1 :]).all()
    assert np.array_equal(chikou, close)

    ref = reference_ichimoku(high, low, close, tenkan, kijun, senkou)
    for got, want in zip((tenkan_sen, kijun_sen, span_a, span_b, chikou), ref):
        assert np.allclose(got, want, equal_nan=True, atol=1e-12)


def test_ichimoku_custom_periods():
    high, low, close = make_ohlc(seed=7)
    ik = Ichimoku(high=high, low=low, close=close, tenkan=5, kijun=10, senkou=20)
    tenkan_sen, kijun_sen, span_a, span_b, _ = ik.compute()
    assert np.isnan(tenkan_sen[:4]).all()
    assert np.isnan(kijun_sen[:9]).all()
    assert np.isnan(span_a[:9]).all()
    assert np.isnan(span_b[:19]).all()
    assert np.allclose(span_a[9:], 0.5 * (tenkan_sen[9:] + kijun_sen[9:]))


def test_ichimoku_chunk_invariance():
    high, low, close = make_ohlc(seed=11)
    full = Ichimoku(high=high, low=low, close=close)
    ref = full.compute()

    chunked = Ichimoku()
    split = len(close) // 4
    for start in range(0, len(close), split):
        chunked.extend(high[start:start + split], low[start:start + split], close[start:start + split])
    for got, want in zip(chunked.compute(), ref):
        assert np.array_equal(got, want, equal_nan=True)


def test_ichimoku_value_and_reset():
    high, low, close = make_ohlc(n=120)
    ik = Ichimoku(tenkan=9, kijun=26, senkou=52)
    assert ik.value is None
    for i in range(60):
        ik.append(high[i], low[i], close[i])
    assert ik.value is not None
    assert ik.value[0] == pytest.approx(ik.compute()[0][59])
    ik.reset()
    assert ik.value is None


def test_ichimoku_rejects_bad_inputs():
    high, low, close = make_ohlc(n=100)
    ik = Ichimoku()
    with pytest.raises(ValueError):
        ik.extend(high[:-1], low, close)
    with pytest.raises(ValueError):
        ik.extend(high, low, close[:-1])
    with pytest.raises(ValueError):
        Ichimoku(tenkan=0)
    with pytest.raises(ValueError):
        Ichimoku(kijun=0)
    with pytest.raises(ValueError):
        Ichimoku(senkou=0)
