"""Compatibility class for causal swing confirmation."""

from typing import Any

from .swing_high_low import SwingHighLow


class SwingHigh(SwingHighLow):
    """Expose the shared causal swing-high/low state under its legacy name."""

    def append(self, high: float, low: float) -> "SwingHigh":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "SwingHigh":
        super().extend(high, low)
        return self

    def reset(self) -> "SwingHigh":
        super().reset()
        return self
