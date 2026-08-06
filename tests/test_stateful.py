"""Regression tests for stateful indicators against the batch API.

The batch API is separately checked against the C TA-Lib oracle.  These tests
make sure append/extend state transitions preserve that compatibility.
"""

import numpy as np
import pytest
import talib as original_talib
from taflow import talib as ta

from taflow.talib import (
    ATR,
    DEMA,
    EMA,
    MACD,
    MIDPOINT,
    MIDPRICE,
    MOM,
    NATR,
    ROC,
    ROCP,
    ROCR,
    ROCR100,
    RSI,
    SMA,
    TEMA,
    TRANGE,
    TRIMA,
    WMA,
)
from taflow.talib import state


def close_data(n=128):
    index = np.arange(n, dtype=np.float64)
    return 100.0 + index * 0.15 + np.sin(index * 0.31) * 4.0


@pytest.mark.parametrize(
    ("class_", "batch", "period"),
    [
        (state.SMA, SMA, 7),
        (state.EMA, EMA, 7),
        (state.WMA, WMA, 7),
        (state.DEMA, DEMA, 7),
        (state.TEMA, TEMA, 7),
        (state.TRIMA, TRIMA, 7),
        (state.KAMA, ta.KAMA, 7),
        (
            lambda period: state.T3(period, 0.7),
            lambda values, period: ta.T3(values, period, 0.7),
            7,
        ),
        (state.MIDPOINT, MIDPOINT, 7),
        (state.RSI, RSI, 14),
        (state.CMO, ta.CMO, 14),
        (state.MOM, MOM, 7),
        (state.ROC, ROC, 7),
        (state.ROCP, ROCP, 7),
        (state.ROCR, ROCR, 7),
        (state.ROCR100, ROCR100, 7),
        (state.MAX, ta.MAX, 7),
        (state.MAXINDEX, ta.MAXINDEX, 7),
        (state.MIN, ta.MIN, 7),
        (state.MININDEX, ta.MININDEX, 7),
        (state.SUM, ta.SUM, 7),
        (state.AVGDEV, ta.AVGDEV, 7),
        (state.LINEARREG, ta.LINEARREG, 14),
        (state.LINEARREG_SLOPE, ta.LINEARREG_SLOPE, 14),
        (state.LINEARREG_INTERCEPT, ta.LINEARREG_INTERCEPT, 14),
        (state.LINEARREG_ANGLE, ta.LINEARREG_ANGLE, 14),
        (state.TSF, ta.TSF, 14),
    ],
)
def test_scalar_extend_and_append_match_batch(class_, batch, period):
    close = close_data()
    expected = batch(close, period)

    indicator = class_(period)
    actual = indicator.extend(close)
    np.testing.assert_allclose(actual, expected, rtol=1e-12, atol=1e-12, equal_nan=True)
    assert indicator.value == pytest.approx(expected[-1], rel=1e-12, abs=1e-12)

    indicator.reset()
    replayed = np.asarray([indicator.append(value) for value in close], dtype=object)
    replayed = np.asarray([np.nan if value is None else value for value in replayed], dtype=np.float64)
    np.testing.assert_allclose(replayed, expected, rtol=1e-12, atol=1e-12, equal_nan=True)


def test_atr_extend_and_chunked_append_match_batch():
    close = close_data()
    high = close + 1.25
    low = close - 0.75
    expected = ATR(high, low, close, 14)

    indicator = state.ATR(14)
    first = indicator.extend(high[:39], low[:39], close[:39])
    second = indicator.extend(high[39:], low[39:], close[39:])
    actual = np.concatenate((first, second))
    np.testing.assert_allclose(actual, expected, rtol=1e-12, atol=1e-12, equal_nan=True)


def test_midprice_extend_and_append_match_batch():
    close = close_data()
    high = close + 1.0 + np.arange(close.size) % 3 * 0.2
    low = close - 0.8 - np.arange(close.size) % 4 * 0.15
    expected = MIDPRICE(high, low, 7)
    indicator = state.MIDPRICE(7)
    actual = indicator.extend(high, low)
    np.testing.assert_allclose(actual, expected, rtol=1e-12, atol=1e-12, equal_nan=True)
    indicator.reset()
    replayed = [indicator.append(h, l) for h, l in zip(high, low)]
    replayed = np.asarray([np.nan if value is None else value for value in replayed])
    np.testing.assert_allclose(replayed, expected, rtol=1e-12, atol=1e-12, equal_nan=True)


@pytest.mark.parametrize(
    "name",
    [
        "ACOS",
        "ASIN",
        "ATAN",
        "CEIL",
        "COS",
        "COSH",
        "EXP",
        "FLOOR",
        "LN",
        "LOG10",
        "SIN",
        "SINH",
        "SQRT",
        "TAN",
        "TANH",
    ],
)
def test_unary_transform_state_matches_batch(name):
    values = np.linspace(0.1, 0.9, 128)
    expected = getattr(ta, name)(values)
    indicator = getattr(state, name)()
    actual = indicator.extend(values)
    np.testing.assert_array_equal(actual, expected)
    assert indicator.value == expected[-1]
    indicator.reset()
    assert indicator.value is None
    np.testing.assert_array_equal([indicator.append(value) for value in values], expected)


@pytest.mark.parametrize("name", ["ADD", "SUB", "MULT", "DIV"])
def test_binary_operator_state_matches_batch(name):
    left = close_data()
    right = np.linspace(1.0, 3.0, left.size)
    expected = getattr(ta, name)(left, right)
    indicator = getattr(state, name)()
    np.testing.assert_array_equal(indicator.extend(left, right), expected)
    indicator.reset()
    np.testing.assert_array_equal(
        [indicator.append(a, b) for a, b in zip(left, right)], expected
    )


def test_price_transform_states_match_batch():
    close = close_data()
    open_ = close - 0.2
    high = close + 1.0
    low = close - 0.8
    cases = [
        ("AVGPRICE", (open_, high, low, close)),
        ("MEDPRICE", (high, low)),
        ("TYPPRICE", (high, low, close)),
        ("WCLPRICE", (high, low, close)),
    ]
    for name, inputs in cases:
        expected = getattr(ta, name)(*inputs)
        indicator = getattr(state, name)()
        np.testing.assert_array_equal(indicator.extend(*inputs), expected)
        indicator.reset()
        np.testing.assert_array_equal(
            [indicator.append(*values) for values in zip(*inputs)], expected
        )


@pytest.mark.parametrize("name", ["MINMAX", "MINMAXINDEX"])
def test_paired_rolling_extrema_match_batch_with_ties(name):
    values = np.asarray(
        [4, 2, 2, 5, 3, 3, 5, 1, 1, 4, 4, 2, 6, 6, 0, 0, 5],
        dtype=np.float64,
    )
    expected = getattr(ta, name)(values, 4)
    indicator = getattr(state, name)(4)
    actual = indicator.extend(values)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_array_equal(ours, theirs)
    indicator.reset()
    replayed = [indicator.append(value) for value in values]
    if name == "MINMAX":
        replayed = [
            (np.nan, np.nan) if value is None else value for value in replayed
        ]
    replayed = tuple(np.asarray(items) for items in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_array_equal(ours, theirs)


@pytest.mark.parametrize("name", ["VAR", "STDDEV"])
def test_rolling_deviation_state_matches_batch(name):
    index = np.arange(128, dtype=np.float64)
    values = 1_000_000.0 + np.sin(index * 0.23) * 11.0 + index % 5 * 0.125
    expected = getattr(ta, name)(values, 12, 2.0)
    indicator = getattr(state, name)(12, 2.0)
    actual = indicator.extend(values)
    np.testing.assert_array_equal(actual, expected)
    indicator.reset()
    replayed = [indicator.append(value) for value in values]
    replayed = np.asarray([np.nan if value is None else value for value in replayed])
    np.testing.assert_array_equal(replayed, expected)


@pytest.mark.parametrize("name", ["BETA", "CORREL"])
def test_bivariate_statistic_state_matches_batch(name):
    index = np.arange(128, dtype=np.float64)
    market = 80.0 + index * 0.08 + np.sin(index * 0.17) * 3.0
    asset = market * 1.3 + np.cos(index * 0.29) * 2.0
    expected = getattr(ta, name)(market, asset, 10)
    indicator = getattr(state, name)(10)
    actual = indicator.extend(market, asset)
    np.testing.assert_array_equal(actual, expected)
    indicator.reset()
    replayed = [indicator.append(x, y) for x, y in zip(market, asset)]
    replayed = np.asarray([np.nan if value is None else value for value in replayed])
    np.testing.assert_array_equal(replayed, expected)


@pytest.mark.parametrize("name", ["AD", "ADOSC", "OBV"])
def test_volume_state_matches_batch(name):
    index = np.arange(128, dtype=np.float64)
    close = 50.0 + index * 0.06 + np.sin(index * 0.25) * 3.0
    high = close + 1.2
    low = close - 0.8
    high[11::19] = close[11::19]
    low[11::19] = close[11::19]
    volume = 1_000.0 + index % 13 * 37.0
    if name == "OBV":
        inputs = (close, volume)
        indicator = state.OBV()
    elif name == "AD":
        inputs = (high, low, close, volume)
        indicator = state.AD()
    else:
        inputs = (high, low, close, volume)
        indicator = state.ADOSC(3, 10)
    expected = getattr(ta, name)(*inputs)
    actual = indicator.extend(*inputs)
    np.testing.assert_array_equal(actual, expected)
    indicator.reset()
    replayed = [indicator.append(*values) for values in zip(*inputs)]
    replayed = np.asarray([np.nan if value is None else value for value in replayed])
    np.testing.assert_array_equal(replayed, expected)


@pytest.mark.parametrize("name", ["BOP", "WILLR", "AROON", "AROONOSC"])
def test_rolling_ohlc_momentum_state_matches_batch(name):
    index = np.arange(128, dtype=np.float64)
    close = 70.0 + index * 0.04 + np.sin(index * 0.27) * 4.0
    open_ = close + np.cos(index * 0.11) * 0.7
    high = np.maximum(open_, close) + 1.0
    low = np.minimum(open_, close) - 0.8
    high[9::17] = close[9::17]
    low[9::17] = close[9::17]
    if name == "BOP":
        inputs = (open_, high, low, close)
        indicator = state.BOP()
        expected = ta.BOP(*inputs)
    elif name == "WILLR":
        inputs = (high, low, close)
        indicator = state.WILLR(14)
        expected = ta.WILLR(*inputs, 14)
    else:
        inputs = (high, low)
        indicator = getattr(state, name)(14)
        expected = getattr(ta, name)(*inputs, 14)
    actual = indicator.extend(*inputs)
    if name == "AROON":
        for ours, theirs in zip(actual, expected):
            np.testing.assert_array_equal(ours, theirs)
    else:
        np.testing.assert_array_equal(actual, expected)
    indicator.reset()
    replayed = [indicator.append(*values) for values in zip(*inputs)]
    if name == "AROON":
        replayed = [
            (np.nan, np.nan) if value is None else value for value in replayed
        ]
        replayed = tuple(np.asarray(items) for items in zip(*replayed))
        for ours, theirs in zip(replayed, expected):
            np.testing.assert_array_equal(ours, theirs)
    else:
        replayed = np.asarray([np.nan if value is None else value for value in replayed])
        np.testing.assert_array_equal(replayed, expected)


@pytest.mark.parametrize(("class_", "batch", "period"), [(state.TRANGE, TRANGE, None), (state.NATR, NATR, 14)])
def test_true_range_family_matches_batch(class_, batch, period):
    close = close_data()
    high = close + 1.25
    low = close - 0.75
    expected = batch(high, low, close) if period is None else batch(high, low, close, period)
    indicator = class_() if period is None else class_(period)
    actual = indicator.extend(high, low, close)
    np.testing.assert_allclose(actual, expected, rtol=1e-12, atol=1e-12, equal_nan=True)


def test_macd_extend_and_warmup_match_batch():
    close = close_data()
    expected = MACD(close, 12, 26, 9)
    indicator = state.MACD(12, 26, 9)
    actual = indicator.extend(close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-12, atol=1e-12, equal_nan=True)
    assert indicator.value == pytest.approx(
        tuple(values[-1] for values in expected), rel=1e-12, abs=1e-12
    )


@pytest.mark.parametrize("signalperiod", [1, 5, 9])
def test_macdfix_matches_oracle_continuation_and_reset(signalperiod):
    close = close_data(300)
    expected = ta.MACDFIX(close, signalperiod)
    indicator = state.MACDFIX(signalperiod)
    actual = indicator.extend(close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-12, atol=1e-12, equal_nan=True)

    chunked = state.MACDFIX(signalperiod)
    first = chunked.extend(close[:20])
    remaining = chunked.extend(close[20:])
    for ours, theirs in zip(
        tuple(np.concatenate((left, right)) for left, right in zip(first, remaining)),
        expected,
    ):
        np.testing.assert_allclose(ours, theirs, rtol=1e-12, atol=1e-12, equal_nan=True)

    indicator.reset()
    replayed = [indicator.append(value) for value in close]
    replayed = [(np.nan, np.nan, np.nan) if value is None else value for value in replayed]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-12, atol=1e-12, equal_nan=True)


def test_mama_extend_reset_and_warmup_match_oracle():
    close = close_data()
    expected = ta.MAMA(close, 0.5, 0.05)
    indicator = state.MAMA(0.5, 0.05)
    actual = indicator.extend(close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-12, atol=1e-12, equal_nan=True)
    assert indicator.value == pytest.approx(
        tuple(values[-1] for values in expected), rel=1e-12, abs=1e-12
    )
    indicator.reset()
    replayed = [indicator.append(value) for value in close]
    replayed = [(np.nan, np.nan) if value is None else value for value in replayed]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-12, atol=1e-12, equal_nan=True)


@pytest.mark.parametrize("name", ["APO", "PPO"])
@pytest.mark.parametrize("matype", range(9))
def test_price_oscillators_match_oracle_for_every_ma_type(name, matype):
    close = close_data(200)
    expected = getattr(ta, name)(close, 7, 13, matype)
    indicator = getattr(state, name)(7, 13, matype)
    actual = indicator.extend(close)
    np.testing.assert_allclose(actual, expected, rtol=1e-10, atol=1e-12, equal_nan=True)
    indicator.reset()
    replayed = np.asarray(
        [np.nan if (value := indicator.append(input)) is None else value for input in close]
    )
    np.testing.assert_allclose(replayed, expected, rtol=1e-10, atol=1e-12, equal_nan=True)
    assert indicator.value == pytest.approx(expected[-1], rel=1e-10, abs=1e-12)


@pytest.mark.parametrize("matype", range(9))
def test_selectable_ma_matches_oracle_for_every_ma_type(matype):
    close = close_data(200)
    expected = ta.MA(close, 13, matype)
    indicator = state.MA(13, matype)
    actual = indicator.extend(close)
    np.testing.assert_allclose(actual, expected, rtol=1e-10, atol=1e-12, equal_nan=True)
    indicator.reset()
    replayed = np.asarray(
        [np.nan if (value := indicator.append(input)) is None else value for input in close]
    )
    np.testing.assert_allclose(replayed, expected, rtol=1e-10, atol=1e-12, equal_nan=True)


@pytest.mark.parametrize("matype", range(9))
def test_bbands_matches_oracle_for_every_ma_type(matype):
    close = close_data(200)
    expected = ta.BBANDS(close, 13, 2.0, 1.5, matype)
    indicator = state.BBANDS(13, 2.0, 1.5, matype)
    actual = indicator.extend(close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-10, atol=1e-10, equal_nan=True)
    indicator.reset()
    replayed = [indicator.append(input) for input in close]
    replayed = [
        (np.nan, np.nan, np.nan) if value is None else value for value in replayed
    ]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-10, atol=1e-10, equal_nan=True)


def test_accbands_matches_oracle_and_reset_replay():
    close = close_data(200)
    index = np.arange(close.size, dtype=np.float64)
    high = close + 1.0 + np.abs(np.sin(index * 0.17))
    low = close - 1.0 - np.abs(np.cos(index * 0.13))
    expected = ta.ACCBANDS(high, low, close, 13)
    indicator = state.ACCBANDS(13)
    actual = indicator.extend(high, low, close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-10, atol=1e-12, equal_nan=True)
    indicator.reset()
    replayed = [indicator.append(*bar) for bar in zip(high, low, close)]
    replayed = [
        (np.nan, np.nan, np.nan) if value is None else value for value in replayed
    ]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_allclose(ours, theirs, rtol=1e-10, atol=1e-12, equal_nan=True)


@pytest.mark.parametrize(("acceleration", "maximum"), [(0.02, 0.2), (0.03, 0.25)])
def test_sar_matches_oracle_and_reset_replay(acceleration, maximum):
    index = np.arange(300, dtype=np.float64)
    center = 100.0 + np.sin(index * 0.21) * 12.0
    high = center + 1.5
    low = center - 1.2
    expected = ta.SAR(high, low, acceleration, maximum)
    indicator = state.SAR(acceleration, maximum)
    actual = indicator.extend(high, low)
    np.testing.assert_allclose(actual, expected, rtol=1e-12, atol=1e-12, equal_nan=True)
    indicator.reset()
    replayed = np.asarray(
        [np.nan if (value := indicator.append(*bar)) is None else value for bar in zip(high, low)]
    )
    np.testing.assert_allclose(replayed, expected, rtol=1e-12, atol=1e-12, equal_nan=True)


@pytest.mark.parametrize(
    "parameters",
    [
        (0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2),
        (0.0, 0.01, 0.03, 0.02, 0.25, 0.04, 0.03, 0.3),
    ],
)
def test_sarext_matches_oracle_and_reset_replay(parameters):
    index = np.arange(300, dtype=np.float64)
    center = 100.0 + np.sin(index * 0.21) * 12.0
    high = center + 1.5
    low = center - 1.2
    expected = ta.SAREXT(high, low, *parameters)
    indicator = state.SAREXT(*parameters)
    actual = indicator.extend(high, low)
    np.testing.assert_allclose(actual, expected, rtol=1e-12, atol=1e-12, equal_nan=True)
    indicator.reset()
    replayed = np.asarray(
        [np.nan if (value := indicator.append(*bar)) is None else value for bar in zip(high, low)]
    )
    np.testing.assert_allclose(replayed, expected, rtol=1e-12, atol=1e-12, equal_nan=True)


def test_imi_matches_oracle_and_reset_replay():
    index = np.arange(300, dtype=np.float64)
    open_ = 100.0 + np.sin(index * 0.17) * 8.0
    close = open_ + np.cos(index * 0.31) * 1.7
    expected = ta.IMI(open_, close, 14)
    indicator = state.IMI(14)
    actual = indicator.extend(open_, close)
    np.testing.assert_allclose(actual, expected, rtol=1e-10, atol=1e-12, equal_nan=True)
    chunked = state.IMI(14)
    first = chunked.extend(open_[:20], close[:20])
    remaining = chunked.extend(open_[20:], close[20:])
    np.testing.assert_allclose(
        np.concatenate((first, remaining)),
        expected,
        rtol=1e-10,
        atol=1e-12,
        equal_nan=True,
    )
    indicator.reset()
    replayed = np.asarray(
        [np.nan if (value := indicator.append(*bar)) is None else value for bar in zip(open_, close)]
    )
    np.testing.assert_allclose(replayed, expected, rtol=1e-10, atol=1e-12, equal_nan=True)


def test_imi_rejects_unequal_input_lengths():
    with pytest.raises(ValueError):
        state.IMI(14).extend(np.ones(20), np.ones(19))


@pytest.mark.parametrize("matype", range(9))
def test_stochf_matches_oracle_for_every_ma_type(matype):
    close = close_data(500)
    index = np.arange(close.size, dtype=np.float64)
    high = close + 1.0 + np.abs(np.sin(index * 0.17))
    low = close - 0.8 - np.abs(np.cos(index * 0.13))
    expected = ta.STOCHF(high, low, close, 5, 13, matype)

    indicator = state.STOCHF(5, 13, matype)
    actual = indicator.extend(high, low, close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )

    chunked = state.STOCHF(5, 13, matype)
    first = chunked.extend(high[:20], low[:20], close[:20])
    remaining = chunked.extend(high[20:], low[20:], close[20:])
    for ours, theirs in zip(
        (np.concatenate(parts) for parts in zip(first, remaining)), expected
    ):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )

    indicator.reset()
    replayed = [indicator.append(*bar) for bar in zip(high, low, close)]
    replayed = [(np.nan, np.nan) if value is None else value for value in replayed]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )


@pytest.mark.parametrize("slowk_matype", range(9))
@pytest.mark.parametrize("slowd_matype", range(9))
def test_stoch_matches_oracle_for_every_ma_pair(slowk_matype, slowd_matype):
    close = close_data(500)
    index = np.arange(close.size, dtype=np.float64)
    high = close + 1.0 + np.abs(np.sin(index * 0.17))
    low = close - 0.8 - np.abs(np.cos(index * 0.13))
    expected = original_talib.STOCH(
        high, low, close, 5, 13, slowk_matype, 11, slowd_matype
    )

    indicator = state.STOCH(5, 13, slowk_matype, 11, slowd_matype)
    actual = indicator.extend(high, low, close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )

    chunked = state.STOCH(5, 13, slowk_matype, 11, slowd_matype)
    first = chunked.extend(high[:20], low[:20], close[:20])
    remaining = chunked.extend(high[20:], low[20:], close[20:])
    for ours, theirs in zip(
        (np.concatenate(parts) for parts in zip(first, remaining)), expected
    ):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )

    indicator.reset()
    replayed = [indicator.append(*bar) for bar in zip(high, low, close)]
    replayed = [(np.nan, np.nan) if value is None else value for value in replayed]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )


def test_stochastic_flat_range_matches_zero_convention():
    values = np.full(100, 42.0)
    for constructor, oracle in (
        (lambda: state.STOCH(5, 3, 0, 3, 0), original_talib.STOCH),
        (lambda: state.STOCHF(5, 3, 0), original_talib.STOCHF),
    ):
        expected = oracle(values, values, values)
        actual = constructor().extend(values, values, values)
        for ours, theirs in zip(actual, expected):
            np.testing.assert_allclose(ours, theirs, equal_nan=True)


@pytest.mark.parametrize("matype", range(9))
def test_stochrsi_matches_oracle_for_every_ma_type(matype):
    close = close_data(500)
    expected = original_talib.STOCHRSI(close, 14, 5, 13, matype)
    indicator = state.STOCHRSI(14, 5, 13, matype)
    actual = indicator.extend(close)
    for ours, theirs in zip(actual, expected):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )

    chunked = state.STOCHRSI(14, 5, 13, matype)
    first = chunked.extend(close[:30])
    remaining = chunked.extend(close[30:])
    for ours, theirs in zip(
        (np.concatenate(parts) for parts in zip(first, remaining)), expected
    ):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )

    indicator.reset()
    replayed = [indicator.append(input) for input in close]
    replayed = [(np.nan, np.nan) if value is None else value for value in replayed]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    for ours, theirs in zip(replayed, expected):
        np.testing.assert_allclose(
            ours, theirs, rtol=1e-8, atol=1e-10, equal_nan=True
        )


def test_rsi_and_kama_edge_conventions_match_oracle():
    flat = np.full(100, 42.0)
    np.testing.assert_array_equal(state.RSI(14).extend(flat), original_talib.RSI(flat, 14))
    close = close_data(100)
    np.testing.assert_array_equal(state.KAMA(1).extend(close), original_talib.KAMA(close, 1))


def test_stochastic_states_reject_unequal_input_lengths():
    with pytest.raises(ValueError):
        state.STOCH(5, 13, 0, 11, 0).extend(
            np.ones(20), np.ones(19), np.ones(20)
        )
    with pytest.raises(ValueError):
        state.STOCHF(5, 13, 0).extend(
            np.ones(20), np.ones(19), np.ones(20)
        )


def test_invalid_parameters_are_rejected():
    with pytest.raises(ValueError):
        state.SMA(0)
    with pytest.raises(ValueError):
        state.EMA(0)
    with pytest.raises(ValueError):
        state.RSI(1)
    with pytest.raises(ValueError):
        state.IMI(1)
    with pytest.raises(ValueError):
        state.DEMA(1)
    with pytest.raises(ValueError):
        state.TEMA(1)
    with pytest.raises(ValueError):
        state.ATR(0)
    with pytest.raises(ValueError):
        state.MACD(1, 26, 9)
    with pytest.raises(ValueError):
        state.MACDFIX(0)
    with pytest.raises(ValueError):
        state.MA(13, 9)
    with pytest.raises(ValueError):
        state.BBANDS(13, 2.0, 2.0, 9)
    with pytest.raises(ValueError):
        state.BBANDS(1, 2.0, 2.0, 0)
    with pytest.raises(ValueError):
        state.STOCHF(0, 3, 0)
    with pytest.raises(ValueError):
        state.STOCHF(5, 0, 0)
    with pytest.raises(ValueError):
        state.STOCHF(5, 3, 9)
    with pytest.raises(ValueError):
        state.STOCH(0, 3, 0, 3, 0)
    with pytest.raises(ValueError):
        state.STOCH(5, 0, 0, 3, 0)
    with pytest.raises(ValueError):
        state.STOCH(5, 3, 0, 0, 0)
    with pytest.raises(ValueError):
        state.STOCH(5, 3, 9, 3, 0)
    with pytest.raises(ValueError):
        state.STOCH(5, 3, 0, 3, 9)
    with pytest.raises(ValueError):
        state.STOCHRSI(1, 5, 3, 0)
    with pytest.raises(ValueError):
        state.STOCHRSI(14, 0, 3, 0)
    with pytest.raises(ValueError):
        state.STOCHRSI(14, 5, 0, 0)
    with pytest.raises(ValueError):
        state.STOCHRSI(14, 5, 3, 9)
