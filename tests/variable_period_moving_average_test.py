"""Correctness and lifecycle tests for VariablePeriodMovingAverage."""

import numpy as np
import pytest
import talib

from taflow import VariablePeriodMovingAverage


def series(length: int = 256) -> tuple[np.ndarray, np.ndarray]:
    rng = np.random.default_rng(20260809)
    values = 100.0 + np.cumsum(rng.normal(0.05, 1.0, length))
    periods = rng.integers(2, 31, length).astype(np.float64)
    return values, periods


@pytest.mark.parametrize("average_type", range(9))
def test_variable_period_moving_average_matches_talib(average_type: int) -> None:
    values, periods = series()
    actual = VariablePeriodMovingAverage(min_period=2, max_period=30, average_type=average_type).extend(values, periods).compute()
    expected = talib.MAVP(
        values,
        periods,
        minperiod=2,
        maxperiod=30,
        matype=average_type,
    )
    np.testing.assert_allclose(actual, expected, rtol=1e-12, atol=1e-12, equal_nan=True)


def test_variable_period_moving_average_lifecycle_is_bitwise_invariant() -> None:
    values, periods = series(400)
    expected = VariablePeriodMovingAverage().extend(values, periods).compute()

    chunked = VariablePeriodMovingAverage().extend(values[:0], periods[:0])
    assert chunked.extend(values[:137], periods[:137]) is chunked
    assert chunked.extend(values[137:], periods[137:]) is chunked
    np.testing.assert_array_equal(chunked.compute().view(np.uint64), expected.view(np.uint64))

    scalar = VariablePeriodMovingAverage().extend(values[:0], periods[:0])
    for value, period in zip(values, periods):
        assert scalar.append(float(value), float(period)) is scalar
    np.testing.assert_array_equal(scalar.compute().view(np.uint64), expected.view(np.uint64))
    assert scalar.value == chunked.value

    assert scalar.reset() is scalar
    assert len(scalar) == 0
    assert scalar.value is None
    scalar.extend(values, periods)
    np.testing.assert_array_equal(scalar.compute().view(np.uint64), expected.view(np.uint64))


def test_variable_period_moving_average_rejects_misaligned_input_before_mutation() -> None:
    state = VariablePeriodMovingAverage()
    with pytest.raises(ValueError, match="equal lengths"):
        state.extend([1.0, 2.0], [2.0])
    assert len(state) == 0
    assert state.value is None
