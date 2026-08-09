"""Persistent highest-since-condition interface."""

from typing import Any

from .._condition_value_adapter import ConditionValueAdapter
from .._native import HighestSinceOperator


class HighestSince(ConditionValueAdapter):
    """Track the maximum input since the latest true condition, inclusively.

    ``condition`` and ``_input`` are required, aligned histories. A true bar
    resets the running maximum to that bar's input. Calculation is in Rust.
    """

    _native_cls = HighestSinceOperator

    def append(self, condition: bool, _input: float) -> "HighestSince":
        super().append(condition, _input)
        return self

    def extend(self, condition: Any, _input: Any) -> "HighestSince":
        super().extend(condition, _input)
        return self

    def reset(self) -> "HighestSince":
        super().reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed condition/input pairs."""
        return super().__len__()
