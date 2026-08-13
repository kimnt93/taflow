import numpy as np

from taflow import BreakOfStructureChangeOfCharacter


def test_break_of_structure_change_of_character_lifecycle() -> None:
    close = 100.0 + np.sin(np.arange(128.0) / 7.0)
    high, low = close + 1.0, close - 1.0
    state = BreakOfStructureChangeOfCharacter(swing_length=3).extend(high, low, close)
    first = state.compute()
    state.reset().extend(high, low, close)
    for got, expected in zip(state.compute(), first):
        np.testing.assert_array_equal(got, expected)
    assert len(state) == len(close)

