"""Canonical native-backed Volume Price Trend adapter."""

from typing import Any

import numpy as np

from .._native import VolumePriceTrendOperator as _Native
from .._series import as_float64_series


class VolumePriceTrend:
    """Compute cumulative Volume Price Trend from close and volume.

    The first aligned output is zero. Each later output adds
    ``volume * (close - previous_close) / previous_close`` to the cumulative
    line; a zero previous close contributes zero. Rust owns arithmetic,
    history, and processed-bar count. The independent oracle/name mapping is
    Wickra ``VolumePriceTrend``.

    Args:
        close: Required chronological close-price history.
        volume: Required aligned volume history.

    Raises:
        ValueError: If the two histories have different lengths or cannot be
            converted to contiguous one-dimensional float64 arrays.
    """

    def __init__(self, close: Any, volume: Any) -> None:
        """Initialize the native state and process aligned history."""
        self._state = _Native()
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "VolumePriceTrend":
        """Append one close/volume observation and return this adapter."""
        self._state.append(float(close), float(volume))
        return self

    def extend(self, close: Any, volume: Any) -> "VolumePriceTrend":
        """Append aligned close and volume histories in one native call.

        Args:
            close: Chronological close-price series.
            volume: Volume series aligned one-for-one with ``close``.

        Returns:
            This instance, allowing method chaining.

        Raises:
            ValueError: If the converted series have different lengths.
        """
        close_values = as_float64_series(close)
        volume_values = as_float64_series(volume)
        if close_values.shape != volume_values.shape:
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(close_values, volume_values)
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned cumulative history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest cumulative value, or ``None`` when empty."""
        return self._state.value

    def reset(self) -> "VolumePriceTrend":
        """Reset native state and clear history, then return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)


__all__ = ["VolumePriceTrend"]
