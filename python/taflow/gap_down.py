"""Persistent downside-gap relation."""

from typing import Any

from ._bar_relation_adapter import BarRelationAdapter
from ._native import GapDownOperator


class GapDown(BarRelationAdapter):
    """Return 1 when a bar's high is below the preceding bar's low.

    ``high`` and ``low`` are required aligned series. The first output is
    ``NaN`` because no preceding bar exists. All calculations run in Rust.
    """

    _native_cls = GapDownOperator

    def append(self, high: float, low: float) -> "GapDown":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "GapDown":
        super().extend(high, low)
        return self

    def reset(self) -> "GapDown":
        super().reset()
        return self
