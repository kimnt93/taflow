"""Compatibility class for causal swing confirmation."""

from typing import Any

from .swing_high_low import SwingHighLow


class SwingLow(SwingHighLow):
    """Expose the shared causal swing-high/low state under its legacy name."""

    def append(self, high: float, low: float) -> "SwingLow":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "SwingLow":
        super().extend(high, low)
        return self

    def reset(self) -> "SwingLow":
        super().reset()
        return self
