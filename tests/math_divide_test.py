import numpy as np
import talib

from taflow import MathDivide


def test_math_divide_matches_talib_and_lifecycle() -> None:
    left = np.linspace(-5.0, 7.0, 128)
    right = np.linspace(3.0, 0.5, 128)
    expected = talib.DIV(left, right)
    actual = MathDivide(left, right)
    np.testing.assert_array_equal(actual.compute(), expected)

    state = MathDivide([], [])
    assert state.extend(left[:51], right[:51]) is state
    assert state.extend(left[51:], right[51:]) is state
    np.testing.assert_array_equal(state.compute(), expected)
    assert state.reset() is state
    for x, y in zip(left, right):
        assert state.append(float(x), float(y)) is state
    np.testing.assert_array_equal(state.compute(), expected)

