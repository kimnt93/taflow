import numpy as np
import pytest
import talib

from taflow import RateOfChangeRatioPercent


@pytest.mark.parametrize("timeperiod", [1, 2, 5, 14, 30])
def test_rate_of_change_ratio_percent_matches_talib_and_lifecycle(
    timeperiod: int,
) -> None:
    rng = np.random.default_rng(5500 + timeperiod)
    values = rng.normal(100.0, 7.0, 257)
    values[[17, 81, 163]] = 0.0
    expected = talib.ROCR100(values, timeperiod=timeperiod)
    actual = RateOfChangeRatioPercent(values, timeperiod)
    np.testing.assert_array_equal(actual.compute(), expected)

    chunked = RateOfChangeRatioPercent([], timeperiod)
    assert chunked.extend(values[:43]).extend(values[43:]) is chunked
    np.testing.assert_array_equal(chunked.compute(), actual.compute())
    assert chunked.reset() is chunked
    for value in values:
        assert chunked.append(float(value)) is chunked
    np.testing.assert_array_equal(chunked.compute(), actual.compute())
    assert chunked.value == actual.value
    assert len(chunked) == len(values)


def test_rate_of_change_ratio_percent_requires_values_and_positive_period() -> None:
    with pytest.raises(ValueError):
        RateOfChangeRatioPercent(None)
    with pytest.raises(ValueError):
        RateOfChangeRatioPercent([], 0)
