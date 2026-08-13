"""Canonical native-backed Chande Momentum Oscillator adapter."""

from typing import Any
import numpy as np
from .._native import ChandeMomentumOscillator as _NativeChandeMomentumOscillator
from .._series import as_float64_series


class ChandeMomentumOscillator:
    """Compute CMO from required ``values`` with period 14 by default."""

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _NativeChandeMomentumOscillator(timeperiod)

    def append(self, value: float) -> "ChandeMomentumOscillator":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "ChandeMomentumOscillator":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "ChandeMomentumOscillator":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["ChandeMomentumOscillator"]
