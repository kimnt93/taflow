"""Pointwise subtract transform."""

from typing import Any

from .._math_state import MathBinaryState
from .._native import MathSubtract as _NativeMathSubtract


class MathSubtract(MathBinaryState):
    """Apply the pointwise subtract operation in persistent Rust state.

    Construction accepts required aligned left and right operand series. This class maps to TA-Lib `SUB`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = _NativeMathSubtract

    def append(self, left: float, right: float) -> "MathSubtract":
        """Append one observation and return this indicator."""
        super().append(left, right)
        return self

    def extend(self, left: Any, right: Any) -> "MathSubtract":
        """Append aligned histories and return this indicator."""
        super().extend(left, right)
        return self

    def reset(self) -> "MathSubtract":
        """Reset native state and return this indicator."""
        super().reset()
        return self
