"""Canonical native-backed Kalman hedge-ratio adapter."""
from typing import Any
import numpy as np
from ._native import KalmanHedgeRatioOperator as _Native
from ._series import as_float64_series


class KalmanHedgeRatio:
    """Online Kalman estimate of beta in y = alpha + beta*x.
    delta controls process noise and observation_variance controls observation
    noise. Outputs are causal.
    """
    def __init__(self, x: Any, y: Any, delta: float = 1e-4,
                 observation_variance: float = 1e-3) -> None:
        self._state = _Native(float(delta), float(observation_variance)); self._length = 0
        self.extend(x, y)
    def append(self, x: float, y: float) -> "KalmanHedgeRatio":
        self._state.append(float(x), float(y)); self._length += 1; return self
    def extend(self, x: Any, y: Any) -> "KalmanHedgeRatio":
        x_values = as_float64_series(x); y_values = as_float64_series(y)
        if x_values.shape != y_values.shape: raise ValueError("x and y must have equal lengths")
        self._state.extend(x_values, y_values); self._length += len(x_values); return self
    def compute(self) -> np.ndarray:
        return self._state.compute()
    @property
    def value(self) -> float | None:
        return self._state.value
    @property
    def alpha(self) -> float | None:
        return self._state.alpha
    @property
    def innovation(self) -> float | None:
        return self._state.innovation
    @property
    def std(self) -> float | None:
        return self._state.std
    def reset(self) -> "KalmanHedgeRatio":
        self._state.reset(); self._length = 0; return self
    def __len__(self) -> int:
        return self._length
