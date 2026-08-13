"""Pointwise divide transform."""

from typing import Any

from .._math_state import MathBinaryState
from .._native import MathDivide as _NativeMathDivide


class MathDivide(MathBinaryState):
    """Apply the pointwise divide operation in persistent Rust state.

    Construction creates a fresh empty state; supply aligned left and right
    operands through ``extend`` or ``append``. This class maps to TA-Lib `DIV`;
    aligned history has no rolling warm-up beyond the native operation's domain
    rules.
    """

    _native_cls = _NativeMathDivide

    def append(self, left: float, right: float) -> "MathDivide":
        """Append one observation and return this indicator."""
        super().append(left, right)
        return self

    def extend(self, left: Any, right: Any) -> "MathDivide":
        """Append aligned histories and return this indicator."""
        super().extend(left, right)
        return self

    def reset(self) -> "MathDivide":
        """Reset native state and return this indicator."""
        super().reset()
        return self
