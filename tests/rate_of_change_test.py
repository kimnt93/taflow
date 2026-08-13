import numpy as np
import pytest
import talib

from taflow import RateOfChange


@pytest.mark.parametrize("timeperiod", [1, 2, 5, 14, 30])
def test_rate_of_change_matches_talib_and_lifecycle(timeperiod: int) -> None:
    rng = np.random.default_rng(5200 + timeperiod)
    values = rng.normal(100.0, 7.0, 257)
    values[[17, 81, 163]] = 0.0
    expected = talib.ROC(values, timeperiod=timeperiod)
    actual = RateOfChange(timeperiod).extend(values)
    np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=1e-12)

    chunked = RateOfChange(timeperiod)
    assert chunked.extend(values[:43]).extend(values[43:]) is chunked
    np.testing.assert_array_equal(chunked.compute(), actual.compute())
    assert chunked.reset() is chunked
    for value in values:
        assert chunked.append(float(value)) is chunked
    np.testing.assert_array_equal(chunked.compute(), actual.compute())
    assert chunked.value == actual.value
    assert len(chunked) == len(values)


def test_rate_of_change_requires_values_and_positive_period() -> None:
    with pytest.raises(ValueError):
        RateOfChange().extend(None)
    with pytest.raises(ValueError):
        RateOfChange(0)
