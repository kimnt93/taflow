"""Persistent lower-low relation."""

from typing import Any

from ._bar_relation_adapter import BarRelationAdapter
from ._native import LowerLowOperator


class LowerLow(BarRelationAdapter):
    """Return 1 when a bar's low is below the preceding bar's low.

    ``high`` and ``low`` are required aligned series. The first output is
    ``NaN`` because no preceding bar exists. All calculations run in Rust.
    """

    _native_cls = LowerLowOperator

    def append(self, high: float, low: float) -> "LowerLow":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "LowerLow":
        super().extend(high, low)
        return self

    def reset(self) -> "LowerLow":
        super().reset()
        return self
