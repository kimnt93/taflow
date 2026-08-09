"""Canonical causal swing-low adapter."""

from typing import Any

from .swing_high_low import SwingHighLow


class SwingLow(SwingHighLow):
    """Expose the confirmed low component of the causal swing state.

    ``high`` and ``low`` are required aligned histories; empty arrays create a
    fresh stream. The shared native state confirms a swing only after the
    configured centered window, and lifecycle methods remain fluent.
    """

    def append(self, high: float, low: float) -> "SwingLow":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "SwingLow":
        super().extend(high, low)
        return self

    def reset(self) -> "SwingLow":
        super().reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return super().__len__()
