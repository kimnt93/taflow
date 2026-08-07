import numpy as np
import pytest

from taflow import Squeeze, SqueezePro


def make_ohlc(n=600, seed=3):
    rng = np.random.default_rng(seed)
    high = 100.0 + np.cumsum(rng.normal(0.0, 0.4, n))
    high += rng.random(n) * 0.5
    low = high - rng.random(n) - 0.2
    close = (high + low) / 2.0 + rng.normal(0.0, 0.1, n)
    return high, low, close


def rolling_sma(x, first_valid, win):
    """Pandas rolling(win).mean() with a leading NaN block (min_periods=win)."""
    n = len(x)
    out = np.full(n, np.nan)
    cum = np.cumsum(np.nan_to_num(x))
    for i in range(first_valid + win - 1, n):
        out[i] = (cum[i] - cum[i - win]) / win
    return out


def pop_std(x, win):
    n = len(x)
    out = np.full(n, np.nan)
    cum = np.cumsum(x)
    cum2 = np.cumsum(x * x)
    for i in range(win - 1, n):
        lo = i + 1 - win
        total = cum[i] - (cum[lo - 1] if lo > 0 else 0.0)
        total2 = cum2[i] - (cum2[lo - 1] if lo > 0 else 0.0)
        mean = total / win
        var = total2 / win - mean * mean
        out[i] = np.sqrt(max(var, 0.0))
    return out


def true_range(high, low, close):
    n = len(close)
    tr = np.full(n, np.nan)
    tr[1:] = np.maximum(
        high[1:] - low[1:],
        np.maximum(np.abs(high[1:] - close[:-1]), np.abs(low[1:] - close[:-1])),
    )
    return tr


def bbands(close, length, std):
    mid = rolling_sma(close, 0, length)
    dev = pop_std(close, length) * std
    return mid - dev, mid + dev


def keltner(high, low, close, length, scalar):
    basis = rolling_sma(close, 0, length)
    tr = true_range(high, low, close)
    band = rolling_sma(tr, 1, length)
    return basis - scalar * band, basis + scalar * band


def reference_squeeze(high, low, close, bb_length, bb_std, kc_length, kc_scalar, mom_length, mom_smooth):
    """Exact transcription of pandas-ta classic `momentum/squeeze.py` (non-lazybear,
    mamode='sma', tr=True)."""
    bb_lower, bb_upper = bbands(close, bb_length, bb_std)
    kc_lower, kc_upper = keltner(high, low, close, kc_length, kc_scalar)
    mom = np.full(len(close), np.nan)
    mom[mom_length:] = close[mom_length:] - close[:-mom_length]
    squeeze = rolling_sma(mom, mom_length, mom_smooth)
    on = (bb_lower > kc_lower) & (bb_upper < kc_upper)
    off = (bb_lower < kc_lower) & (bb_upper > kc_upper)
    no = ~on & ~off
    return squeeze, on.astype(float), off.astype(float), no.astype(float)


def reference_squeeze_pro(high, low, close, bb_length, bb_std, kc_length, scalars, mom_length, mom_smooth):
    bb_lower, bb_upper = bbands(close, bb_length, bb_std)
    kc = [keltner(high, low, close, kc_length, scalar) for scalar in scalars]
    mom = np.full(len(close), np.nan)
    mom[mom_length:] = close[mom_length:] - close[:-mom_length]
    squeeze = rolling_sma(mom, mom_length, mom_smooth)
    on_wide = (bb_lower > kc[0][0]) & (bb_upper < kc[0][1])
    on_normal = (bb_lower > kc[1][0]) & (bb_upper < kc[1][1])
    on_narrow = (bb_lower > kc[2][0]) & (bb_upper < kc[2][1])
    off = (bb_lower < kc[0][0]) & (bb_upper > kc[0][1])
    no = ~on_wide & ~off
    return (
        squeeze,
        on_wide.astype(float),
        on_normal.astype(float),
        on_narrow.astype(float),
        off.astype(float),
        no.astype(float),
    )


def test_squeeze_warmup_and_reference():
    high, low, close = make_ohlc()
    bb_length, bb_std, kc_length, kc_scalar, mom_length, mom_smooth = 20, 2.0, 20, 1.5, 12, 6
    sq = Squeeze(high=high, low=low, close=close)
    squeeze, on, off, no = sq.compute()

    assert np.isnan(squeeze[: mom_length + mom_smooth - 1]).all()
    assert np.isfinite(squeeze[mom_length + mom_smooth - 1 :]).all()
    assert np.isin(on[20:], (0.0, 1.0)).all()
    assert np.isin(off[20:], (0.0, 1.0)).all()
    assert np.isin(no[20:], (0.0, 1.0)).all()
    for i in range(20, len(close)):
        assert on[i] + off[i] + no[i] == 1.0
    assert on[:20].sum() == 0 and off[:20].sum() == 0 and no[:20].sum() == 20

    ref = reference_squeeze(high, low, close, bb_length, bb_std, kc_length, kc_scalar, mom_length, mom_smooth)
    assert np.allclose(squeeze, ref[0], equal_nan=True, atol=1e-12)
    for got, want in zip((on, off, no), ref[1:]):
        assert np.array_equal(got, want)


def test_squeeze_custom_params():
    high, low, close = make_ohlc(seed=7)
    sq = Squeeze(high=high, low=low, close=close, bb_length=10, bb_std=1.0, kc_length=15, kc_scalar=2.0, mom_length=8, mom_smooth=3)
    squeeze, on, off, no = sq.compute()
    ref = reference_squeeze(high, low, close, 10, 1.0, 15, 2.0, 8, 3)
    assert np.allclose(squeeze, ref[0], equal_nan=True, atol=1e-12)
    for got, want in zip((on, off, no), ref[1:]):
        assert np.array_equal(got, want)


def test_squeeze_chunk_invariance():
    high, low, close = make_ohlc(seed=11)
    full = Squeeze(high=high, low=low, close=close)
    ref = full.compute()

    chunked = Squeeze()
    split = len(close) // 4
    for start in range(0, len(close), split):
        chunked.extend(high[start:start + split], low[start:start + split], close[start:start + split])
    for got, want in zip(chunked.compute(), ref):
        assert np.array_equal(got, want, equal_nan=True)


def test_squeeze_value_and_reset():
    high, low, close = make_ohlc(n=120)
    sq = Squeeze()
    assert sq.value is None
    for i in range(60):
        sq.append(high[i], low[i], close[i])
    assert sq.value is not None
    assert sq.value[0] == pytest.approx(sq.compute()[0][59])
    sq.reset()
    assert sq.value is None


def test_squeeze_rejects_bad_inputs():
    high, low, close = make_ohlc(n=100)
    sq = Squeeze()
    with pytest.raises(ValueError):
        sq.extend(high[:-1], low, close)
    with pytest.raises(ValueError):
        sq.extend(high, low, close[:-1])
    with pytest.raises(ValueError):
        Squeeze(bb_length=0)
    with pytest.raises(ValueError):
        Squeeze(bb_std=0.0)
    with pytest.raises(ValueError):
        Squeeze(kc_length=0)
    with pytest.raises(ValueError):
        Squeeze(kc_scalar=0.0)
    with pytest.raises(ValueError):
        Squeeze(mom_length=0)
    with pytest.raises(ValueError):
        Squeeze(mom_smooth=0)


def test_squeeze_pro_warmup_and_reference():
    high, low, close = make_ohlc(seed=5)
    scalars = (2.0, 1.5, 1.0)
    sp = SqueezePro(high=high, low=low, close=close)
    squeeze, on_wide, on_normal, on_narrow, off, no = sp.compute()

    assert np.isnan(squeeze[:17]).all()
    assert np.isfinite(squeeze[17:]).all()
    assert np.isin(on_wide[20:], (0.0, 1.0)).all()
    assert np.isin(on_normal[20:], (0.0, 1.0)).all()
    assert np.isin(on_narrow[20:], (0.0, 1.0)).all()
    assert np.isin(off[20:], (0.0, 1.0)).all()
    assert np.isin(no[20:], (0.0, 1.0)).all()

    ref = reference_squeeze_pro(high, low, close, 20, 2.0, 20, scalars, 12, 6)
    assert np.allclose(squeeze, ref[0], equal_nan=True, atol=1e-12)
    for got, want in zip((on_wide, on_normal, on_narrow, off, no), ref[1:]):
        assert np.array_equal(got, want)


def test_squeeze_pro_classification_monotonic():
    high, low, close = make_ohlc(seed=13)
    _, on_wide, on_normal, on_narrow, off, no = SqueezePro(high=high, low=low, close=close).compute()
    assert (on_wide[20:] >= on_normal[20:]).all()
    assert (on_normal[20:] >= on_narrow[20:]).all()
    for i in range(20, len(close)):
        if on_wide[i] == 1.0:
            assert no[i] == 0.0 and off[i] == 0.0


def test_squeeze_pro_rejects_bad_inputs():
    high, low, close = make_ohlc(n=100)
    sp = SqueezePro()
    with pytest.raises(ValueError):
        sp.extend(high[:-1], low, close)
    with pytest.raises(ValueError):
        SqueezePro(kc_scalar_wide=1.0, kc_scalar_normal=1.5)
    with pytest.raises(ValueError):
        SqueezePro(kc_scalar_wide=2.0, kc_scalar_normal=2.0)
    with pytest.raises(ValueError):
        SqueezePro(kc_scalar_normal=0.0)
