"""Canonical native-backed signed-power adapter."""
from typing import Any
import numpy as np
from .._native import SignedPowerOperator as _Native
from .._series import as_float64_series


class SignedPower:
    """Pointwise sign(x) times abs(x) raised to exponent."""
    def __init__(self, input: Any, exponent: float = 2.0) -> None:
        self._state = _Native(float(exponent)); self._length = 0; self.extend(input)
    def append(self, input: float) -> "SignedPower":
        self._state.append(float(input)); self._length += 1; return self
    def extend(self, input: Any) -> "SignedPower":
        values = as_float64_series(input); self._state.extend(values); self._length += len(values); return self
    def compute(self) -> np.ndarray: return self._state.compute()
    @property
    def value(self) -> float | None: return self._state.value
    def reset(self) -> "SignedPower": self._state.reset(); self._length = 0; return self
    def __len__(self) -> int: return self._length
