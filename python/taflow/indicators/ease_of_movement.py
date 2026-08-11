"""Native-backed Ease of Movement adapter."""

from typing import Any

import numpy as np

from .._native import EaseOfMovementOperator as _Native
from .._series import as_float64_series


class EaseOfMovement:
    """Average range-scaled midpoint movement per unit of volume.

    ``high``, ``low``, and ``volume`` are required equal-length chronological
    series and may all be empty for a fresh stream. Rust owns midpoint change,
    rolling mean, warm-up, and aligned output. The raw movement is
    ``midpoint_change * range * divisor / volume`` and is averaged over
    ``period`` bars. Wickra ``EaseOfMovement`` is the oracle. ``compute`` returns one float
    array, ``value`` is the latest scalar or ``None`` during warm-up, and
    lifecycle mutators return ``self``.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        volume: Any,
        period: int = 14,
        divisor: float = 100_000_000.0,
    ) -> None:
        """Initialize aligned histories, averaging period, and output scale."""
        self._state = _Native(period, divisor)
        self.extend(high, low, volume)

    def append(self, high: float, low: float, volume: float) -> "EaseOfMovement":
        """Append one high/low/volume bar and return this adapter."""
        self._state.append(float(high), float(low), float(volume))
        return self

    def extend(self, high: Any, low: Any, volume: Any) -> "EaseOfMovement":
        """Append equal-length high, low, and volume histories."""
        arrays = tuple(as_float64_series(series) for series in (high, low, volume))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, and volume must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Ease of Movement history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest movement value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "EaseOfMovement":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)
