"""Canonical native-backed BOS/CHOCH adapter."""
from typing import Any
import numpy as np
from .._native import BreakOfStructureChangeOfCharacterOperator as _Native
from .._series import as_float64_series


class BreakOfStructureChangeOfCharacter:
    """Causal BOS and CHOCH events over required high, low, and close series.
    swing_length controls swing confirmation; outputs are bos, choch, level,
    broken with NaN until an event exists.
    """
    def __init__(self, high: Any, low: Any, close: Any, swing_length: int = 5) -> None:
        self._state = _Native(int(swing_length)); self._length = 0
        self.extend(high, low, close)
    def append(self, high: float, low: float, close: float) -> "BreakOfStructureChangeOfCharacter":
        self._state.append(float(high), float(low), float(close)); self._length += 1; return self
    def extend(self, high: Any, low: Any, close: Any) -> "BreakOfStructureChangeOfCharacter":
        high_values = as_float64_series(high); low_values = as_float64_series(low); close_values = as_float64_series(close)
        if not (high_values.shape == low_values.shape == close_values.shape): raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(high_values, low_values, close_values); self._length += len(high_values); return self
    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()
    @property
    def value(self) -> tuple[float, float, float, float] | None:
        return self._state.value
    def reset(self) -> "BreakOfStructureChangeOfCharacter":
        self._state.reset(); self._length = 0; return self
    def __len__(self) -> int:
        return self._length
