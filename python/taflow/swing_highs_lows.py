"""Canonical name for causal swing high and low confirmation."""

from typing import Any

from .swing import SwingHighLow


class SwingHighsLows(SwingHighLow):
    """Canonical plural name for the causal swing confirmation state."""

    def append(self, high: float, low: float) -> "SwingHighsLows":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "SwingHighsLows":
        super().extend(high, low)
        return self

    def reset(self) -> "SwingHighsLows":
        super().reset()
        return self

__all__ = ["SwingHighsLows"]
