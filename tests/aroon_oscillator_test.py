import numpy as np
import pytest
import talib

from taflow import AroonOscillator


@pytest.mark.parametrize("timeperiod", [2, 5, 14, 30])
def test_aroon_oscillator_matches_talib_matrix_and_lifecycle(timeperiod: int) -> None:
    rng = np.random.default_rng(66047 + timeperiod)
    for base in [
        rng.normal(size=257).cumsum(),
        np.full(257, 8.0),
        np.arange(257.0),
        np.resize(np.array([1.0, 3.0, 3.0, 2.0, 1.0]), 257),
    ]:
        high = base + 1.0
        low = base - 1.0
        expected = talib.AROONOSC(high, low, timeperiod)
        actual = AroonOscillator(timeperiod).extend(high, low)
        np.testing.assert_allclose(actual.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True)

        state = AroonOscillator(timeperiod)
        assert state.extend(high[:47], low[:47]) is state
        assert state.extend(high[47:], low[47:]) is state
        np.testing.assert_array_equal(state.compute(), actual.compute())
        assert state.reset() is state
        for index in range(len(high)):
            assert state.append(float(high[index]), float(low[index])) is state
        np.testing.assert_array_equal(state.compute(), actual.compute())


def test_aroon_oscillator_validates_configuration_and_alignment_before_mutation() -> None:
    with pytest.raises(ValueError):
        AroonOscillator(1)
    state = AroonOscillator(5)
    with pytest.raises(ValueError):
        state.extend([1.0, 2.0], [1.0])
    assert len(state) == 0
    assert state.value is None
