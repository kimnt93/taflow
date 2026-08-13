"""Canonical native-backed Kalman hedge-ratio adapter."""
from typing import Any
import numpy as np
from .._native import KalmanHedgeRatioOperator as _Native
from .._series import as_float64_series


class KalmanHedgeRatio:
    """Estimate the causal hedge ratio in ``y = alpha + beta * x``.

    The two-state random-walk Kalman filter uses transition variance
    ``delta / (1 - delta)`` and the supplied observation variance. It emits
    beta from the first pair; the zero diffuse prior therefore emits ``0`` on
    that first observation. Rust owns state, history, and arithmetic. The
    independent oracle/name mapping is Wickra ``KalmanHedgeRatio`` with its
    ``a`` target mapped to ``y`` and ``b`` regressor mapped to ``x``.

    Args:
        x: Required chronological regressor series.
        y: Required aligned target series.
        delta: State-drift ratio in ``(0, 1)``; default ``1e-4``.
        observation_variance: Positive measurement-noise variance; default
            ``1e-3``.

    Raises:
        ValueError: If inputs are misaligned or configuration is invalid.
    """
    def __init__(self, delta: float = 1e-4,
                 observation_variance: float = 1e-3) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(float(delta), float(observation_variance))
    def append(self, x: float, y: float) -> "KalmanHedgeRatio":
        """Append one regressor/target pair and return this adapter."""
        self._state.append(float(x), float(y))
        return self
    def extend(self, x: Any, y: Any) -> "KalmanHedgeRatio":
        x_values = as_float64_series(x); y_values = as_float64_series(y)
        if x_values.shape != y_values.shape:
            raise ValueError("x and y must have equal lengths")
        self._state.extend(x_values, y_values)
        return self
    def compute(self) -> np.ndarray:
        """Return the aligned native beta history."""
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
        """Reset the filter and clear history, then return this adapter."""
        self._state.reset()
        return self
    def __len__(self) -> int:
        """Return the processed-pair count delegated to native state."""
        return len(self._state)
