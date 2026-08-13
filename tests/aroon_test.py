import numpy as np
import pytest
import talib

from taflow import Aroon


def _assert_lifecycle(high: np.ndarray, low: np.ndarray, timeperiod: int) -> None:
    expected_down, expected_up = talib.AROON(high, low, timeperiod)
    actual = Aroon(timeperiod).extend(high, low)
    actual_down, actual_up = actual.compute()
    np.testing.assert_allclose(actual_down, expected_down, rtol=1e-12, atol=1e-12, equal_nan=True)
    np.testing.assert_allclose(actual_up, expected_up, rtol=1e-12, atol=1e-12, equal_nan=True)

    state = Aroon(timeperiod)
    split = max(1, len(high) // 3)
    assert state.extend(high[:split], low[:split]) is state
    assert state.extend(high[split:], low[split:]) is state
    down, up = state.compute()
    np.testing.assert_array_equal(down, actual_down)
    np.testing.assert_array_equal(up, actual_up)
    assert state.value == actual.value
    assert state.reset() is state
    assert state.value is None
    for index in range(len(high)):
        assert state.append(float(high[index]), float(low[index])) is state
    down, up = state.compute()
    np.testing.assert_array_equal(down, actual_down)
    np.testing.assert_array_equal(up, actual_up)


@pytest.mark.parametrize("timeperiod", [2, 5, 14, 30])
def test_aroon_matches_talib_matrix_and_lifecycle(timeperiod: int) -> None:
    rng = np.random.default_rng(55103 + timeperiod)
    random = rng.normal(size=257).cumsum()
    cases = [
        random,
        np.full(257, 8.0),
        np.arange(257.0),
        np.arange(257.0, 0.0, -1.0),
        np.resize(np.array([1.0, 3.0, 3.0, 2.0, 1.0]), 257),
    ]
    for base in cases:
        _assert_lifecycle(base + 1.0, base - 1.0, timeperiod)


def test_aroon_validates_configuration_and_alignment_before_mutation() -> None:
    with pytest.raises(ValueError):
        Aroon(1)
    state = Aroon(5)
    with pytest.raises(ValueError):
        state.extend([1.0, 2.0], [1.0])
    assert len(state) == 0
    assert state.value is None
