"""Persistent inside-bar relation."""

from typing import Any

from ._bar_relation_adapter import BarRelationAdapter
from ._native import InsideBarOperator


class InsideBar(BarRelationAdapter):
    """Return 1 when a bar is strictly inside the preceding range.

    ``high`` and ``low`` are required aligned series. The first output is
    ``NaN`` because no preceding bar exists. All calculations run in Rust.
    """

    _native_cls = InsideBarOperator

    def append(self, high: float, low: float) -> "InsideBar":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "InsideBar":
        super().extend(high, low)
        return self

    def reset(self) -> "InsideBar":
        super().reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return super().__len__()
