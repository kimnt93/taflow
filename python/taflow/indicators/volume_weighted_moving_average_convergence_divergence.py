from typing import Any

import numpy as np

from .._native import VolumeWeightedMovingAverageConvergenceDivergence as _Native
from .._series import as_float64_series


class VolumeWeightedMovingAverageConvergenceDivergence:
    """Compute VWMA MACD, its EMA signal, and histogram in native Rust.

    Output order is ``(convergence_divergence, signal, histogram)`` and maps to
    Wickra ``VolumeWeightedMacd``. Warm-up is represented by aligned ``NaN``.

    Args:
        close: Required chronological closing-price history.
        volume: Required volume history aligned with ``close``.
        fast: Fast volume-weighted window. Defaults to 12.
        slow: Slow volume-weighted window. Defaults to 26.
        signal: EMA signal period. Defaults to 9.

    Raises:
        ValueError: If inputs differ in length or periods are invalid.
    """

    def __init__(self, fast: int = 12, slow: int = 26, signal: int = 9) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(fast, slow, signal)

    def append(
        self, close: float, volume: float
    ) -> "VolumeWeightedMovingAverageConvergenceDivergence":
        """Append one close/volume observation and return this adapter."""
        self._state.append(float(close), float(volume))
        return self

    def extend(
        self, close: Any, volume: Any
    ) -> "VolumeWeightedMovingAverageConvergenceDivergence":
        """Append aligned close and volume histories after length validation."""
        close_array = as_float64_series(close)
        volume_array = as_float64_series(volume)
        if len(close_array) != len(volume_array):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(close_array, volume_array)
        return self

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest three outputs, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned convergence-divergence, signal, and histogram arrays."""
        return self._state.compute()

    def reset(self) -> "VolumeWeightedMovingAverageConvergenceDivergence":
        """Clear all rolling and EMA state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
