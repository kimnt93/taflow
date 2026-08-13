import numpy as np
import pytest
import talib

from taflow import MovingAverage


@pytest.mark.parametrize("timeperiod", [1, 2, 7, 30])
@pytest.mark.parametrize("matype", range(9))
def test_matches_talib(timeperiod: int, matype: int) -> None:
    rng = np.random.default_rng(51231 + timeperiod * 17 + matype)
    values = 100.0 + rng.normal(size=257).cumsum()
    expected = talib.MA(values, timeperiod=timeperiod, matype=matype)
    actual = MovingAverage(timeperiod, matype).extend(values).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-9, equal_nan=True)


def test_lifecycle_is_bitwise_invariant() -> None:
    values = np.linspace(10.0, 90.0, 431) + np.sin(np.arange(431) * 0.13)
    batch = MovingAverage(17, 2).extend(values)
    chunked = MovingAverage(17, 2)
    assert chunked.extend(values[:53]) is chunked
    assert chunked.extend(values[53:]) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())
    assert chunked.value == batch.value
    assert chunked.reset() is chunked
    for value in values:
        assert chunked.append(float(value)) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())
    assert len(chunked) == len(values)


def test_input_and_parameter_validation() -> None:
    with pytest.raises(ValueError):
        MovingAverage().extend(None)
    with pytest.raises(ValueError):
        MovingAverage().extend([[1.0, 2.0]])
    with pytest.raises(ValueError):
        MovingAverage(0)
    with pytest.raises(ValueError):
        MovingAverage(5, 99)
