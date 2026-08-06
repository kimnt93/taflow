import numpy as np
import talib as original_talib

import taflow
from taflow import talib


def price_data(length=300):
    index = np.arange(length, dtype=np.float64)
    close = 100.0 + np.sin(index * 0.21) * 12.0 + index * 0.01
    high = close + 1.5
    low = close - 1.2
    return high, low, close


def assert_outputs_equal(actual, expected, rtol=1e-10, atol=1e-12):
    if isinstance(expected, tuple):
        for ours, theirs in zip(actual, expected):
            np.testing.assert_allclose(
                ours, theirs, rtol=rtol, atol=atol, equal_nan=True
            )
    else:
        np.testing.assert_allclose(
            actual, expected, rtol=rtol, atol=atol, equal_nan=True
        )


def test_talib_compatibility_namespace_preserves_old_style_functions():
    high, low, close = price_data()
    open_ = close - np.cos(np.arange(close.size, dtype=np.float64) * 0.31)
    assert_outputs_equal(talib.MA(close, 13, 1), original_talib.MA(close, 13, 1))
    assert_outputs_equal(
        talib.BBANDS(close, 13, 2.0, 1.5, 0),
        original_talib.BBANDS(close, 13, 2.0, 1.5, 0),
    )
    assert_outputs_equal(
        talib.ACCBANDS(high, low, close, 13),
        original_talib.ACCBANDS(high, low, close, 13),
    )
    assert_outputs_equal(talib.SAR(high, low), original_talib.SAR(high, low))
    assert_outputs_equal(talib.SAREXT(high, low), original_talib.SAREXT(high, low))
    assert_outputs_equal(talib.IMI(open_, close, 14), original_talib.IMI(open_, close, 14))
    assert_outputs_equal(talib.MACDFIX(close, 9), original_talib.MACDFIX(close, 9))


def test_descriptive_moving_average_and_bollinger_bands():
    _, _, close = price_data()
    moving_average = taflow.MovingAverage(period=13, moving_average_type=1)
    assert_outputs_equal(
        moving_average.extend(close), original_talib.MA(close, 13, 1)
    )
    moving_average.reset()
    replayed = np.asarray(
        [
            np.nan if (value := moving_average.append(input)) is None else value
            for input in close
        ]
    )
    assert_outputs_equal(replayed, original_talib.MA(close, 13, 1))

    bands = taflow.BollingerBands(
        period=13,
        deviations_up=2.0,
        deviations_down=1.5,
        moving_average_type=0,
    )
    assert_outputs_equal(
        bands.extend(close), original_talib.BBANDS(close, 13, 2.0, 1.5, 0)
    )


def test_descriptive_acceleration_and_parabolic_bands():
    high, low, close = price_data()
    acceleration_bands = taflow.AccelerationBands(period=13)
    assert_outputs_equal(
        acceleration_bands.extend(high, low, close),
        original_talib.ACCBANDS(high, low, close, 13),
    )

    sar = taflow.ParabolicSar(acceleration=0.03, maximum=0.25)
    assert_outputs_equal(
        sar.extend(high, low), original_talib.SAR(high, low, 0.03, 0.25)
    )

    parameters = (0.0, 0.01, 0.03, 0.02, 0.25, 0.04, 0.03, 0.3)
    extended = taflow.ParabolicSarExtended(
        start_value=parameters[0],
        offset_on_reverse=parameters[1],
        acceleration_init_long=parameters[2],
        acceleration_long=parameters[3],
        acceleration_max_long=parameters[4],
        acceleration_init_short=parameters[5],
        acceleration_short=parameters[6],
        acceleration_max_short=parameters[7],
    )
    assert_outputs_equal(
        extended.extend(high, low), original_talib.SAREXT(high, low, *parameters)
    )


def test_descriptive_intraday_momentum_index():
    _, _, close = price_data()
    open_ = close - np.cos(np.arange(close.size, dtype=np.float64) * 0.31)
    indicator = taflow.IntradayMomentumIndex(period=14)
    expected = original_talib.IMI(open_, close, 14)
    assert_outputs_equal(indicator.extend(open_, close), expected)
    indicator.reset()
    replayed = np.asarray(
        [np.nan if (value := indicator.append(*bar)) is None else value for bar in zip(open_, close)]
    )
    assert_outputs_equal(replayed, expected)


def test_descriptive_fixed_macd():
    _, _, close = price_data()
    indicator = taflow.MovingAverageConvergenceDivergenceFixed(signal_period=9)
    expected = original_talib.MACDFIX(close, 9)
    assert_outputs_equal(indicator.extend(close), expected, rtol=1e-12, atol=1e-12)
    indicator.reset()
    replayed = [indicator.append(value) for value in close]
    replayed = [(np.nan, np.nan, np.nan) if value is None else value for value in replayed]
    replayed = tuple(np.asarray(values) for values in zip(*replayed))
    assert_outputs_equal(replayed, expected, rtol=1e-12, atol=1e-12)


def test_descriptive_classes_are_defined_in_individual_modules():
    assert taflow.MovingAverage.__module__ == "taflow.moving_average"
    assert taflow.BollingerBands.__module__ == "taflow.bollinger_bands"
    assert taflow.AccelerationBands.__module__ == "taflow.acceleration_bands"
    assert taflow.IntradayMomentumIndex.__module__ == "taflow.intraday_momentum_index"
    assert (
        taflow.MovingAverageConvergenceDivergenceFixed.__module__
        == "taflow.moving_average_convergence_divergence_fixed"
    )
    assert taflow.ParabolicSar.__module__ == "taflow.parabolic_sar"
    assert (
        taflow.ParabolicSarExtended.__module__
        == "taflow.parabolic_sar_extended"
    )
