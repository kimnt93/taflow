"""Canonical native-backed fractional-differencing adapter."""
from typing import Any
import numpy as np
from .._native import FracDiffOperator as _Native
from .._series import as_float64_series


class FracDiff:
    """Fixed-width fractional differencing of a required input series.
    d is the differencing order and threshold truncates small weights.
    Scalar output is None during warm-up; history uses NaN there.
    """
    def __init__(self, input: Any, d: float = 0.5, threshold: float = 1e-5) -> None:
        self._state = _Native(float(d), float(threshold)); self._length = 0
        self.extend(input)
    def append(self, input: float) -> "FracDiff":
        self._state.append(float(input)); self._length += 1; return self
    def extend(self, input: Any) -> "FracDiff":
        values = as_float64_series(input); self._state.extend(values)
        self._length += len(values); return self
    def compute(self) -> np.ndarray:
        return self._state.compute()
    @property
    def value(self) -> float | None:
        return self._state.value
    def reset(self) -> "FracDiff":
        self._state.reset(); self._length = 0; return self
    def __len__(self) -> int:
        return self._length
