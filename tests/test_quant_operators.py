import numpy as np
import pytest

from taflow import Adv, Amihud, Cusum, OuHalfLife, RollSpread


def make_close_volume(n=500):
    rng = np.random.default_rng(7)
    close = 100.0 + np.cumsum(rng.normal(0.0, 0.2, n))
    volume = rng.lognormal(14.0, 0.5, n)
    return close, volume


def test_adv_warmup_and_definition():
    close, volume = make_close_volume()
    period = 20
    adv = Adv(timeperiod=period).extend(close, volume).compute()
    assert np.isnan(adv[: period - 1]).all()
    assert np.isfinite(adv[period:]).all()
    assert (adv[period:] >= 0).all()
    manual = np.full(len(close), np.nan)
    cum = 0.0
    for i in range(len(close)):
        cum += close[i] * volume[i]
        if i >= period - 1:
            if i >= period:
                cum -= close[i - period] * volume[i - period]
            manual[i] = cum / period
    assert np.allclose(adv, manual, equal_nan=True)


def test_amihud_warmup_and_definition():
    close, volume = make_close_volume()
    period = 20
    amihud = Amihud(timeperiod=period).extend(close, volume).compute()
    assert np.isnan(amihud[:period]).all()
    assert np.isfinite(amihud[period:]).all()
    assert (amihud[period:] >= 0).all()
    manual = np.full(len(close), np.nan)
    window = []
    for i in range(1, len(close)):
        illiq = abs(close[i] / close[i - 1] - 1.0) / (close[i] * volume[i])
        window.append(illiq)
        if len(window) > period:
            window.pop(0)
        if len(window) == period:
            manual[i] = sum(window) / period
    assert np.allclose(amihud, manual, equal_nan=True)


def test_roll_spread_definition():
    rng = np.random.default_rng(11)
    close = 100.0 + np.cumsum(rng.normal(0.0, 0.1, 500))
    period = 20
    spread = RollSpread(timeperiod=period).extend(close).compute()
    assert np.isnan(spread[:period]).all()
    assert np.isfinite(spread[period:]).all()
    assert (spread[period:] >= 0).all()
    manual = np.full(len(close), np.nan)
    delta = np.diff(close)
    for i in range(period, len(close)):
        xs = [delta[t] for t in range(i - period, i)]
        ys = [delta[t - 1] if t > 0 else 0.0 for t in range(i - period, i)]
        mean_x = sum(xs) / period
        mean_y = sum(ys) / period
        cov = sum((x - mean_x) * (y - mean_y) for x, y in zip(xs, ys)) / (period - 1)
        manual[i] = 2.0 * np.sqrt(max(0.0, -cov))
    assert np.allclose(spread, manual, equal_nan=True)


def test_ou_half_life_definition():
    rng = np.random.default_rng(3)
    close = 100.0 + np.cumsum(rng.normal(0.0, 0.1, 500))
    period = 20
    halflife = OuHalfLife(timeperiod=period).extend(close).compute()
    assert np.isnan(halflife[:period]).all()
    assert (~np.isnan(halflife[period:])).any()
    manual = np.full(len(close), np.nan)
    delta = np.diff(close)
    for i in range(period, len(close)):
        xs = delta[i - period : i]
        ys = close[i - period : i]
        mean_x = xs.mean()
        mean_y = ys.mean()
        cov = np.sum((xs - mean_x) * (ys - mean_y)) / (period - 1)
        var_y = np.sum((ys - mean_y) ** 2) / (period - 1)
        if var_y > 0.0:
            lam = -cov / var_y
            manual[i] = np.log(2.0) / lam if lam > 0.0 else np.nan
    assert np.allclose(halflife, manual, equal_nan=True)


def test_cusum_detects_drift():
    change = np.zeros(300)
    change[200:] = 1.0
    cusum = Cusum(threshold=1.0).extend(change).compute()
    assert (cusum[:200] == 0).all()
    assert set(cusum[200:]) <= {-1.0, 0.0, 1.0}
    assert (cusum[200:] != 0).any()
    assert cusum[200] == 0.0
    assert cusum[201] == 1.0


def test_quant_operators_reset_and_reject_bad_periods():
    close, volume = make_close_volume(n=100)

    for cls, bad in [(Adv, (close[:-1], volume)), (Amihud, (close, volume[:-1]))]:
        state = cls()
        with pytest.raises(ValueError):
            state.extend(*bad)
        state.reset()
        assert len(state.compute()) == 0

    with pytest.raises(ValueError):
        Adv(timeperiod=0)
    with pytest.raises(ValueError):
        RollSpread(timeperiod=0)
    with pytest.raises(ValueError):
        OuHalfLife(timeperiod=0)

    rng = np.random.default_rng(0)
    price = 100.0 + np.cumsum(rng.normal(0.0, 0.1, 50))
    rs = RollSpread().extend(price)
    rs.reset()
    assert len(rs.compute()) == 0
    rs.extend(price)
    assert len(rs.compute()) == 50
    assert rs.value is not None


def test_append_matches_extend():
    close, volume = make_close_volume(n=200)
    period = 20

    adv_append = Adv(timeperiod=period)
    for c, v in zip(close, volume):
        adv_append.append(c, v)
    assert np.allclose(adv_append.compute(), Adv(timeperiod=period).extend(close, volume).compute(), equal_nan=True)

    cusum_append = Cusum()
    for c in np.diff(close):
        cusum_append.append(c)
    assert np.allclose(cusum_append.compute(), Cusum().extend(np.diff(close)).compute())
