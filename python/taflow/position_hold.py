"""Persistent position-hold adapter."""

from typing import Any

import numpy as np

from ._native import PositionHoldOperator
from ._series import as_float64_series


class PositionHold:
    """Hold the most recent non-zero position value.

    ``position`` is a required numeric history; an empty array creates a fresh
    stream. Every zero input carries the previous position, while non-zero
    inputs replace it. Lifecycle methods are fluent, ``value`` returns the
    latest scalar or ``None``, and ``compute`` returns a NumPy history.
    """

    def __init__(
        self,
        position: Any,
    ) -> None:
        """Create the native state and replay the position history."""
        self._state = PositionHoldOperator()
        self.extend(position)

    def append(self, position: float) -> "PositionHold":
        """Append one position and return this adapter."""
        self._state.append(float(position))
        return self

    def extend(self, position: Any) -> "PositionHold":
        """Append a numeric position history and return this adapter."""
        self._state.extend(as_float64_series(position))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete held-position history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest held position or ``None`` for an empty stream."""
        return self._state.value

    def reset(self) -> "PositionHold":
        """Reset native state and output history, returning this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed positions."""
        return len(self._state.compute())
