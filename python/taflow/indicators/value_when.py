"""Persistent value-at-condition interface."""

from typing import Any

from .._condition_value_adapter import ConditionValueAdapter
from .._native import ValueWhenOperator


class ValueWhen(ConditionValueAdapter):
    """Carry forward the latest input whose aligned condition was true.

    ``condition`` and ``_input`` are required, aligned histories. Output is
    ``NaN`` until the first true condition. The persistent calculation is Rust.
    """

    _native_cls = ValueWhenOperator

    def append(self, condition: bool, _input: float) -> "ValueWhen":
        super().append(condition, _input)
        return self

    def extend(self, condition: Any, _input: Any) -> "ValueWhen":
        super().extend(condition, _input)
        return self

    def reset(self) -> "ValueWhen":
        super().reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed condition/input pairs."""
        return super().__len__()
