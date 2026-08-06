"""Lifecycle tests for the unified persistent indicator API."""

import numpy as np
import pytest

from taflow import EMA, ExponentialMovingAverage
from taflow.indicators import EMA as NamespacedEMA
from taflow.talib import EMA as batch_ema


class SeriesLike:
    def __init__(self, values):
        self._values = values

    def to_numpy(self):
        return np.asarray(self._values)


class DataFrameLike:
    def __init__(self, **columns):
        self._columns = columns
        self.columns = list(columns)

    def __getitem__(self, name):
        return SeriesLike(self._columns[name])


def test_ema_unified_lifecycle_continues_without_replay():
    history = np.linspace(10.0, 30.0, 41)
    continuation = [31.0, 32.0, 33.0]
    expected = batch_ema(np.append(history, continuation), 7)

    indicator = ExponentialMovingAverage(history, 7)
    initial = indicator.compute()
    np.testing.assert_allclose(initial, expected[: history.size], equal_nan=True)

    indicator.append(continuation[0]).extend(continuation[1:])
    np.testing.assert_allclose(indicator.compute(), expected, equal_nan=True)
    assert indicator.value == pytest.approx(expected[-1])
    assert len(indicator) == expected.size


def test_ema_aliases_resolve_to_the_same_class():
    assert EMA is ExponentialMovingAverage
    assert NamespacedEMA is ExponentialMovingAverage


@pytest.mark.parametrize(
    "values",
    [
        [1, 2, 3, 4, 5],
        np.arange(1, 6, dtype=np.float32),
        SeriesLike([1, 2, 3, 4, 5]),
    ],
)
def test_ema_accepts_supported_series_inputs(values):
    result = ExponentialMovingAverage(values, 3).compute()
    np.testing.assert_allclose(result, batch_ema(np.arange(1.0, 6.0), 3), equal_nan=True)
    assert result.dtype == np.float64


def test_ema_dataframe_selection_is_explicit_and_transactional():
    frame = DataFrameLike(close=[1, 2, 3, 4], volume=[10, 11, 12, 13])
    with pytest.raises(ValueError, match="column is required"):
        ExponentialMovingAverage(frame, 3)

    indicator = ExponentialMovingAverage(frame, 3, column="close")
    before = indicator.compute()
    with pytest.raises(ValueError, match="one-dimensional"):
        indicator.extend([[5.0], [6.0]])
    np.testing.assert_array_equal(indicator.compute(), before)


def test_ema_reset_clears_state_and_output_history():
    indicator = ExponentialMovingAverage([1, 2, 3, 4], 3)
    indicator.reset()
    assert len(indicator) == 0
    assert indicator.value is None
    assert indicator.compute().size == 0
