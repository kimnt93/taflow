"""Canonical native-backed BOS/CHOCH adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import BreakOfStructureChangeOfCharacterOperator as _Native
from .._series import as_float64_series


class BreakOfStructureChangeOfCharacter:
    """Causal BOS and CHOCH events over required high, low, and close series.
    swing_length controls swing confirmation; outputs are bos, choch, level,
    broken with NaN until an event exists.
    """

    def __init__(self, swing_length: int = 5) -> None:
        self._state = _Native(int(swing_length))

    def append(
        self, high: float, low: float, close: float
    ) -> "BreakOfStructureChangeOfCharacter":
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(
        self, high: Any, low: Any, close: Any
    ) -> "BreakOfStructureChangeOfCharacter":
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        close_values = as_float64_series(close)
        if not (high_values.shape == low_values.shape == close_values.shape):
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(high_values, low_values, close_values)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float, float] | None:
        return self._state.value

    def reset(self) -> "BreakOfStructureChangeOfCharacter":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
