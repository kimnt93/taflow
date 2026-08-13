import numpy as np
import pandas as pd
import pytest
import talib
import wickra

from taflow import RollingMaximumDrawdown


def pandas_maximum_drawdown(equity: np.ndarray, timeperiod: int) -> np.ndarray:
    """Independent pandas rolling-window oracle for non-negative equity."""

    def window_value(window: np.ndarray) -> float:
        peaks = np.maximum.accumulate(window)
        drawdowns = np.divide(
            peaks - window,
            peaks,
            out=np.zeros_like(window),
            where=peaks > 0.0,
        )
        return float(drawdowns.max(initial=0.0))

    return (
        pd.Series(equity, dtype=np.float64)
        .rolling(timeperiod)
        .apply(window_value, raw=True)
        .to_numpy()
    )


@pytest.mark.parametrize("timeperiod", [1, 2, 5, 14, 31])
@pytest.mark.parametrize(
    "case",
    ["random", "constant", "monotonic", "repeated_extrema", "minimum_length"],
)
def test_matches_pandas_oracle(timeperiod: int, case: str) -> None:
    rng = np.random.default_rng(20260810 + timeperiod)
    cases = {
        "random": 100.0 * np.exp(np.cumsum(rng.normal(0.0, 0.02, 160))),
        "constant": np.full(80, 50.0),
        "monotonic": np.linspace(10.0, 100.0, 80),
        "repeated_extrema": np.resize(
            np.array([100.0, 120.0, 120.0, 90.0, 90.0, 110.0]), 90
        ),
        "minimum_length": np.linspace(1.0, 2.0, timeperiod),
    }
    equity = cases[case]
    actual = RollingMaximumDrawdown(timeperiod).extend(equity).compute()
    expected = pandas_maximum_drawdown(equity, timeperiod)
    np.testing.assert_allclose(actual, expected, rtol=1e-14, atol=1e-14, equal_nan=True)


def test_matches_wickra_and_records_talib_unavailability() -> None:
    rng = np.random.default_rng(42)
    equity = 100.0 * np.exp(np.cumsum(rng.normal(0.0, 0.025, 500)))
    equity[[140, 260]] = np.nan
    timeperiod = 21

    actual = RollingMaximumDrawdown(timeperiod).extend(equity).compute()
    expected = np.asarray(wickra.MaxDrawdown(timeperiod).batch(equity), dtype=np.float64)
    np.testing.assert_allclose(actual, expected, rtol=1e-14, atol=1e-14, equal_nan=True)
    assert "MAXDRAWDOWN" not in talib.get_functions()


def test_scalar_chunked_warmed_continuation_and_reset_are_invariant() -> None:
    rng = np.random.default_rng(7)
    equity = 100.0 * np.exp(np.cumsum(rng.normal(0.0, 0.03, 127)))
    period = 17
    batch = RollingMaximumDrawdown(period).extend(equity)
    expected = batch.compute()

    scalar = RollingMaximumDrawdown(period)
    assert scalar.value is None
    for value in equity:
        assert scalar.append(value) is scalar
    np.testing.assert_array_equal(scalar.compute(), expected)

    chunked = RollingMaximumDrawdown(period)
    assert chunked.extend(equity[:13]) is chunked
    chunked.extend(equity[13:81]).extend(equity[81:])
    np.testing.assert_array_equal(chunked.compute(), expected)

    continuation = np.array([95.0, 87.0, 102.0])
    batch.extend(continuation)
    scalar.extend(continuation)
    chunked.extend(continuation)
    np.testing.assert_array_equal(batch.compute(), scalar.compute())
    np.testing.assert_array_equal(batch.compute(), chunked.compute())

    assert batch.reset() is batch
    assert batch.value is None
    assert len(batch) == 0
    batch.extend(equity)
    np.testing.assert_array_equal(batch.compute(), expected)
    assert len(batch) == equity.size
    assert batch.period == period


def test_rejects_zero_period() -> None:
    with pytest.raises(ValueError):
        RollingMaximumDrawdown(0)
