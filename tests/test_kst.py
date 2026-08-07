import numpy as np
import pytest

from taflow import Kst


def make_close(n=700, seed=3):
    rng = np.random.default_rng(seed)
    return 100.0 + np.cumsum(rng.normal(0.0, 0.4, n)) + rng.normal(0.0, 0.1, n)


def reference_kst(close, r1, r2, r3, r4, n1, n2, n3, n4, nsig):
    """Exact transcription of bukosabino `ta` `trend.KSTIndicator`."""
    n = len(close)
    mean = close.mean()

    def rocma(r, m):
        shift = np.concatenate((np.full(r, mean), close[:-r]))
        roc = (close - shift) / shift
        out = np.full(n, np.nan)
        for i in range(m - 1, n):
            window = roc[i + 1 - m : i + 1]
            valid = window[~np.isnan(window)]
            if len(valid) >= m:
                out[i] = valid.mean()
        return out

    kst = 100 * (
        rocma(r1, n1)
        + 2 * rocma(r2, n2)
        + 3 * rocma(r3, n3)
        + 4 * rocma(r4, n4)
    )
    sig = np.full(n, np.nan)
    for i in range(n):
        window = kst[max(0, i + 1 - nsig) : i + 1]
        valid = window[~np.isnan(window)]
        if len(valid) > 0:
            sig[i] = valid.mean()
    return kst, sig


def test_kst_warmup_and_reference():
    close = make_close()
    ks = Kst(close=close)
    kst, signal = ks.compute()

    assert np.isnan(kst[:44]).all()
    assert np.isfinite(kst[44:]).all()
    assert np.isnan(signal[:44]).all()
    assert np.isfinite(signal[52:]).all()

    # The reference fills the ROC shift warm-up with the global close mean;
    # taflow leaves those bars NaN, so outputs match from bar 44 (KST) and
    # bar 52 (signal), where every window contains only real ROC values.
    ref = reference_kst(close, 10, 15, 20, 30, 10, 10, 10, 15, 9)
    assert np.allclose(kst[44:], ref[0][44:], atol=1e-12)
    assert np.allclose(signal[52:], ref[1][52:], atol=1e-12)


def test_kst_custom_params():
    close = make_close(seed=7)
    ks = Kst(close=close, roc1=6, roc2=11, roc3=16, roc4=24, sma1=5, sma2=8, sma3=10, sma4=12, signal=7)
    kst, signal = ks.compute()
    ref = reference_kst(close, 6, 11, 16, 24, 5, 8, 10, 12, 7)
    start = 24 + 12 - 1
    sig_start = start + 7 - 1
    assert np.allclose(kst[start:], ref[0][start:], atol=1e-12)
    assert np.allclose(signal[sig_start:], ref[1][sig_start:], atol=1e-12)
    assert np.isnan(kst[: start - 1]).all()


def test_kst_chunk_invariance():
    close = make_close(seed=11)
    full = Kst(close=close)
    ref = full.compute()

    chunked = Kst()
    split = len(close) // 4
    for start in range(0, len(close), split):
        chunked.extend(close[start:start + split])
    for got, want in zip(chunked.compute(), ref):
        assert np.array_equal(got, want, equal_nan=True)


def test_kst_value_and_reset():
    close = make_close(n=150)
    ks = Kst()
    assert ks.value is None
    for i in range(100):
        ks.append(close[i])
    assert ks.value is not None
    assert ks.value[0] == pytest.approx(ks.compute()[0][99])
    ks.reset()
    assert ks.value is None


def test_kst_rejects_bad_inputs():
    with pytest.raises(ValueError):
        Kst(roc1=0)
    with pytest.raises(ValueError):
        Kst(sma4=0)
    with pytest.raises(ValueError):
        Kst(signal=0)
