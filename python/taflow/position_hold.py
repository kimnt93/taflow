"""Native-backed position-hold adapter."""

from typing import Any

import numpy as np

from ._native import PositionHoldOperator as _Native
from ._series import as_float64_series


class PositionHold:
    """Hold the most recent non-zero position value.

    ``position`` is the required chronological numeric series and may be empty
    for a fresh stream. Each zero carries the previous position, while a
    non-zero input replaces it. ``compute`` returns one aligned float array,
    ``value`` is the latest held position or ``None`` for an empty stream, and
    lifecycle mutators return ``self``. No independent external oracle exists
    for this stateful signal definition.
    """

    def __init__(self, position: Any) -> None:
        self._state = _Native()
        self._length = 0
        self.extend(position)

    def append(self, position: float) -> "PositionHold":
        """Append one position and return this adapter."""
        self._state.append(float(position))
        self._length += 1
        return self

    def extend(self, position: Any) -> "PositionHold":
        """Append a chronological position series and return this adapter."""
        values = as_float64_series(position)
        self._state.extend(values)
        self._length += len(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned held-position history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest held position, or ``None`` for an empty stream."""
        return self._state.value

    def reset(self) -> "PositionHold":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed positions."""
        return self._length


__all__ = ["PositionHold"]
