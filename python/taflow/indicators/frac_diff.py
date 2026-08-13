"""Canonical native-backed fractional-differencing adapter."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import FracDiffOperator as _Native
from .._series import as_float64_series


class FracDiff:
    """Fixed-width fractional differencing of a required input series.
    d is the differencing order and threshold truncates small weights.
    Scalar output is None during warm-up; history uses NaN there.
    """

    def __init__(self, d: float = 0.5, threshold: float = 1e-5) -> None:
        self._state = _Native(float(d), float(threshold))

    def append(self, input: float) -> "FracDiff":
        self._state.append(float(input))
        return self

    def extend(self, input: Any) -> "FracDiff":
        values = as_float64_series(input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self) -> float | None:
        return self._state.value

    def reset(self) -> "FracDiff":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return adapter_length(self)
