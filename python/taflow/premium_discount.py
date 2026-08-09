"""Canonical native-backed PremiumDiscount adapter."""
from typing import Any
import numpy as np
from ._native import PremiumDiscount as _NativePremiumDiscount
from ._series import as_float64_series


class PremiumDiscount:
    """Compute rolling equilibrium and signed premium/discount zones."""
    def __init__(self, close: Any, window: int = 20) -> None:
        self._state = _NativePremiumDiscount(window)
        self.extend(close)

    def append(self, close: float) -> "PremiumDiscount":
        self._state.append(float(close)); return self

    def extend(self, close: Any) -> "PremiumDiscount":
        self._state.extend(as_float64_series(close)); return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]: return self._state.compute()
    @property
    def value(self) -> tuple[int, float] | None: return self._state.value
    def reset(self) -> "PremiumDiscount": self._state.reset(); return self
    def __len__(self) -> int: return len(self._state)


__all__ = ["PremiumDiscount"]
