"""Persistent lowest-since-condition interface."""

from typing import Any

from ._condition_value_adapter import ConditionValueAdapter
from ._native import LowestSinceOperator


class LowestSince(ConditionValueAdapter):
    """Track the minimum input since the latest true condition, inclusively.

    ``condition`` and ``_input`` are required, aligned histories. A true bar
    resets the running minimum to that bar's input. Calculation is in Rust.
    """

    _native_cls = LowestSinceOperator

    def append(self, condition: bool, _input: float) -> "LowestSince":
        super().append(condition, _input)
        return self

    def extend(self, condition: Any, _input: Any) -> "LowestSince":
        super().extend(condition, _input)
        return self

    def reset(self) -> "LowestSince":
        super().reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed condition/input pairs."""
        return super().__len__()
