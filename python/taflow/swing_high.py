"""Canonical causal swing-high adapter."""

from typing import Any

from .swing_high_low import SwingHighLow


class SwingHigh(SwingHighLow):
    """Expose the confirmed high component of the causal swing state.

    ``high`` and ``low`` are required aligned histories; empty arrays create a
    fresh stream. The shared native state confirms a swing only after the
    configured centered window, and lifecycle methods remain fluent.
    """

    def append(self, high: float, low: float) -> "SwingHigh":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "SwingHigh":
        super().extend(high, low)
        return self

    def reset(self) -> "SwingHigh":
        super().reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return super().__len__()
