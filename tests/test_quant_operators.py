import numpy as np
import pytest

from taflow import (
    Adv,
    Amihud,
    Cusum,
    FracDiff,
    KalmanHedgeRatio,
    OuHalfLife,
    RollSpread,
    SpreadZscore,
)


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


def test_spread_zscore_matches_hedge_ratio_composition():
    rng = np.random.default_rng(5)
    x = 100.0 + np.cumsum(rng.normal(0.0, 0.2, 300))
    beta = 1.5
    y = 10.0 + beta * x + rng.normal(0.0, 0.3, len(x))
    period = 20

    z = SpreadZscore(timeperiod=period).extend(x, y).compute()
    assert np.isnan(z[: period - 1]).all()

    from taflow import HedgeRatio

    hedge = HedgeRatio(timeperiod=period).extend(x, y).compute()
    manual = np.full(len(x), np.nan)
    for i in range(period - 1, len(x)):
        window_x = x[i + 1 - period : i + 1]
        window_y = y[i + 1 - period : i + 1]
        spreads = window_y - hedge[i] * window_x
        mean = spreads.mean()
        var = spreads.var()
        manual[i] = (spreads[-1] - mean) / np.sqrt(var) if var > 0 else 0.0
    assert np.allclose(z, manual, equal_nan=True)


def test_spread_zscore_reset_and_bad_inputs():
    rng = np.random.default_rng(9)
    x = 100.0 + np.cumsum(rng.normal(0.0, 0.1, 100))
    y = 1.2 * x + rng.normal(0.0, 0.1, len(x))

    with pytest.raises(ValueError):
        SpreadZscore(timeperiod=0)
    state = SpreadZscore()
    with pytest.raises(ValueError):
        state.extend(x[:-1], y)
    state.reset()
    assert len(state.compute()) == 0

    sz = SpreadZscore().extend(x, y)
    sz.reset()
    assert len(sz.compute()) == 0
    sz.extend(x, y)
    assert len(sz.compute()) == 100
    assert sz.value is not None


def _frac_diff_weights(d, threshold):
    weights = [1.0]
    k = 1
    while True:
        wk = -weights[-1] * (d - k + 1) / k
        if abs(wk) < threshold:
            break
        weights.append(wk)
        k += 1
    return weights


def test_frac_diff_matches_reference_weights():
    rng = np.random.default_rng(13)
    price = 100.0 + np.cumsum(rng.normal(0.0, 0.2, 300))
    d, threshold = 0.5, 1e-3

    fd = FracDiff(d=d, threshold=threshold).extend(price).compute()
    weights = _frac_diff_weights(d, threshold)
    w = len(weights)
    assert np.isnan(fd[: w - 1]).all()

    manual = np.full(len(price), np.nan)
    for i in range(w - 1, len(price)):
        manual[i] = sum(weight * price[i - j] for j, weight in enumerate(weights))
    assert np.allclose(fd, manual, equal_nan=True)


def test_frac_diff_reset_and_bad_params():
    with pytest.raises(ValueError):
        FracDiff(d=0.0)
    with pytest.raises(ValueError):
        FracDiff(threshold=0.0)
    with pytest.raises(ValueError):
        FracDiff(d=-0.5)

    price = np.arange(50.0)
    state = FracDiff().extend(price)
    state.reset()
    assert len(state.compute()) == 0
    state.extend(price)
    assert len(state.compute()) == 50


def test_frac_diff_append_matches_extend():
    rng = np.random.default_rng(17)
    price = 100.0 + np.cumsum(rng.normal(0.0, 0.2, 200))
    fd_append = FracDiff()
    for p in price:
        fd_append.append(p)
    assert np.allclose(fd_append.compute(), FracDiff().extend(price).compute(), equal_nan=True)


def _reference_kalman(x, y, delta, obs_var):
    alpha, beta = 0.0, 1.0
    p = np.eye(2)
    out = np.full(len(x), np.nan)
    for i, (xi, yi) in enumerate(zip(x, y)):
        p_pred = p + delta * np.eye(2)
        h = np.array([1.0, xi])
        pred = np.array([alpha, beta])
        innovation = yi - h @ pred
        s = h @ p_pred @ h + obs_var
        gain = p_pred @ h / s
        pred = pred + gain * innovation
        alpha, beta = pred
        p = (np.eye(2) - np.outer(gain, h)) @ p_pred
        out[i] = beta
    return out


def test_kalman_hedge_ratio_matches_reference():
    rng = np.random.default_rng(23)
    x = 100.0 + np.cumsum(rng.normal(0.0, 0.3, 300))
    y = 2.0 * x + 5.0 + rng.normal(0.0, 0.5, len(x))
    delta, obs_var = 1e-4, 1e-3

    result = KalmanHedgeRatio(delta=delta, observation_variance=obs_var).extend(x, y)
    assert np.allclose(result.compute(), _reference_kalman(x, y, delta, obs_var), atol=1e-6)
    assert np.isfinite(result.alpha)
    assert np.isfinite(result.innovation)
    assert result.std > 0
    assert abs(result.value - result.compute()[-1]) < 1e-12


def test_kalman_hedge_ratio_tracks_synthetic_beta():
    x = np.arange(1, 201, dtype=float) / 10.0
    y = 1.0 + 2.0 * x
    beta = KalmanHedgeRatio(delta=1e-4, observation_variance=1e-3).extend(x, y).compute()
    assert abs(beta[-1] - 2.0) < 0.1

    append_beta = KalmanHedgeRatio()
    for xi, yi in zip(x, y):
        append_beta.append(xi, yi)
    assert np.allclose(append_beta.compute(), KalmanHedgeRatio().extend(x, y).compute())


def test_kalman_hedge_ratio_reset_and_bad_params():
    rng = np.random.default_rng(29)
    x = 100.0 + np.cumsum(rng.normal(0.0, 0.2, 100))
    y = 1.5 * x + rng.normal(0.0, 0.2, len(x))

    with pytest.raises(ValueError):
        KalmanHedgeRatio(delta=-0.1)
    with pytest.raises(ValueError):
        KalmanHedgeRatio(observation_variance=0.0)
    state = KalmanHedgeRatio()
    with pytest.raises(ValueError):
        state.extend(x[:-1], y)
    state.reset()
    assert len(state.compute()) == 0
    state.extend(x, y)
    assert len(state.compute()) == 100
