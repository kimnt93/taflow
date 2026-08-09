"""Native-backed Ease of Movement adapter."""

from typing import Any

import numpy as np

from ._native import EaseOfMovementOperator as _Native
from ._series import as_float64_series


class EaseOfMovement:
    """Compute high-low midpoint movement normalized by volume and range.

    ``high``, ``low``, and ``volume`` are required equal-length chronological
    series and may all be empty for a fresh stream. Rust owns midpoint change,
    box ratio, warm-up, and aligned output. ``compute`` returns one float
    array, ``value`` is the latest scalar or ``None`` during warm-up, and
    lifecycle mutators return ``self``. The oracle is pandas-ta-classic ``eom``.
    """

    def __init__(self, high: Any, low: Any, volume: Any) -> None:
        self._state = _Native()
        self._length = 0
        self.extend(high, low, volume)

    def append(self, high: float, low: float, volume: float) -> "EaseOfMovement":
        """Append one high/low/volume bar and return this adapter."""
        self._state.append(float(high), float(low), float(volume))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any, volume: Any) -> "EaseOfMovement":
        """Append equal-length high, low, and volume histories."""
        arrays = tuple(as_float64_series(series) for series in (high, low, volume))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, and volume must have equal lengths")
        self._state.extend(*arrays)
        self._length += len(arrays[0])
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
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return self._length


__all__ = ["EaseOfMovement"]
