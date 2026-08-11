"""Canonical native-backed Klinger Volume Oscillator adapter."""

from typing import Any

import numpy as np

from .._native import KlingerVolumeOscillator as _NativeKlingerVolumeOscillator
from .._series import as_float64_series


class KlingerVolumeOscillator:
    """Compute signed-volume fast/slow EMA and signal outputs.

    ``high``, ``low``, ``close``, and ``volume`` are required aligned series in
    that order; empty arrays create a fresh stream. ``fast``, ``slow``, and
    ``signal`` default to 34, 55, and 13. Rust owns signed-volume force,
    exponential smoothing, warm-up, and aligned NaN history. ``compute``
    returns ``(oscillator, signal)`` arrays; lifecycle mutators return ``self``
    and ``value`` exposes the latest pair. Wickra ``KVO`` is the correctness
    oracle for the oscillator; Wickra does not expose TAFlow's signal output.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        fast: int = 34,
        slow: int = 55,
        signal: int = 13,
    ) -> None:
        """Initialize native KVO state and process aligned OHLCV histories."""
        self._state = _NativeKlingerVolumeOscillator(int(fast), int(slow), int(signal))
        self.extend(high, low, close, volume)

    def append(
        self, high: float, low: float, close: float, volume: float
    ) -> "KlingerVolumeOscillator":
        """Append one high/low/close/volume bar in that order."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any
    ) -> "KlingerVolumeOscillator":
        """Append aligned high/low/close/volume histories in that order."""
        arrays = tuple(as_float64_series(value) for value in (high, low, close, volume))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, close, and volume must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned oscillator and signal arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest pair, or ``None`` while signal warm-up runs."""
        return self._state.value

    def reset(self) -> "KlingerVolumeOscillator":
        """Reset the state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)


__all__ = ["KlingerVolumeOscillator"]
