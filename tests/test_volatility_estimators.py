import numpy as np
import pytest

from taflow import (
    CloseToCloseSigma,
    GarmanKlass,
    GkYangZhang,
    Parkinson,
    RogersSatchell,
    YangZhang,
)


def make_ohlc(n=500):
    rng = np.random.default_rng(42)
    open_ = 100.0 + np.cumsum(rng.normal(0.0, 0.1, n))
    close = open_ + rng.normal(0.0, 0.5, n)
    high = np.maximum(open_, close) + np.abs(rng.normal(0.0, 0.3, n))
    low = np.minimum(open_, close) - np.abs(rng.normal(0.0, 0.3, n))
    return open_, high, low, close


@pytest.mark.parametrize("period", [10, 20, 50])
def test_volatility_estimators_warmup_and_bounds(period):
    open_, high, low, close = make_ohlc()
    n = len(close)

    close_to_close = CloseToCloseSigma(timeperiod=period).extend(close).compute()
    assert np.isnan(close_to_close[: period]).all()
    assert (close_to_close[period:] >= 0).all()

    parkinson = Parkinson(timeperiod=period).extend(high, low).compute()
    assert np.isnan(parkinson[: period - 1]).all()
    assert (parkinson[period:] >= 0).all()

    gk = GarmanKlass(timeperiod=period).extend(open_, high, low, close).compute()
    assert np.isnan(gk[: period - 1]).all()
    assert (gk[period:] >= 0).all()

    rs = RogersSatchell(timeperiod=period).extend(open_, high, low, close).compute()
    assert np.isnan(rs[: period - 1]).all()
    assert (rs[period:] >= 0).all()

    gk_yz = GkYangZhang(timeperiod=period).extend(open_, high, low, close).compute()
    assert np.isnan(gk_yz[: period]).all()
    assert (gk_yz[period:] >= 0).all()

    yz = YangZhang(timeperiod=period).extend(open_, high, low, close).compute()
    assert np.isnan(yz[: period]).all()
    assert (yz[period:] >= 0).all()

    assert np.allclose(close_to_close[period:], close_to_close[period:])
    assert np.all(np.isfinite(close_to_close[period:]))


def test_volatility_estimators_reset_and_reject_bad_inputs():
    open_, high, low, close = make_ohlc(n=100)

    with pytest.raises(ValueError):
        YangZhang(timeperiod=1)
    with pytest.raises(ValueError):
        Parkinson(timeperiod=0)

    for cls, args in [
        (Parkinson, (high, low)),
        (GarmanKlass, (open_, high, low, close)),
    ]:
        state = cls()
        with pytest.raises(ValueError):
            state.extend(*args[:-1], args[-1][:-1])
        state.reset()
        assert len(state.compute()) == 0
