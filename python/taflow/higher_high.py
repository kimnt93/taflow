"""Persistent higher-high relation."""

from typing import Any

from ._bar_relation_adapter import BarRelationAdapter
from ._native import HigherHighOperator


class HigherHigh(BarRelationAdapter):
    """Return 1 when a bar's high exceeds the preceding bar's high.

    ``high`` and ``low`` are required aligned series. The first output is
    ``NaN`` because no preceding bar exists. All calculations run in Rust.
    """

    _native_cls = HigherHighOperator

    def append(self, high: float, low: float) -> "HigherHigh":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "HigherHigh":
        super().extend(high, low)
        return self

    def reset(self) -> "HigherHigh":
        super().reset()
        return self
