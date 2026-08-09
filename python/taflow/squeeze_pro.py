"""Canonical native-backed Squeeze PRO adapter."""
from typing import Any
import numpy as np
from ._native import SqueezeProOperator as _Native
from ._series import as_float64_series


class SqueezePro:
    """Squeeze PRO over required high, low, and close series.
    Outputs are squeeze, on_wide, on_normal, on_narrow, off, no.
    """
    def __init__(self, high: Any, low: Any, close: Any, bb_length: int = 20,
                 bb_std: float = 2.0, kc_length: int = 20,
                 kc_scalar_wide: float = 2.0, kc_scalar_normal: float = 1.5,
                 kc_scalar_narrow: float = 1.0, mom_length: int = 12,
                 mom_smooth: int = 6) -> None:
        self._state = _Native(int(bb_length), float(bb_std), int(kc_length), float(kc_scalar_wide), float(kc_scalar_normal), float(kc_scalar_narrow), int(mom_length), int(mom_smooth)); self._length = 0
        self.extend(high, low, close)
    def append(self, high: float, low: float, close: float) -> "SqueezePro":
        self._state.append(float(high), float(low), float(close)); self._length += 1; return self
    def extend(self, high: Any, low: Any, close: Any) -> "SqueezePro":
        high_values = as_float64_series(high); low_values = as_float64_series(low); close_values = as_float64_series(close)
        if not (high_values.shape == low_values.shape == close_values.shape): raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(high_values, low_values, close_values); self._length += len(high_values); return self
    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()
    @property
    def value(self) -> tuple[float, float, float, float, float, float] | None:
        return self._state.value
    def reset(self) -> "SqueezePro":
        self._state.reset(); self._length = 0; return self
    def __len__(self) -> int:
        return self._length
