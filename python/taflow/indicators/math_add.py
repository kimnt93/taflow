"""Pointwise add transform."""

from typing import Any

from .._math_state import MathBinaryState
from .._native import MathAdd as _NativeMathAdd


class MathAdd(MathBinaryState):
    """Apply the pointwise add operation in persistent Rust state.

    Construction creates a fresh empty state; supply aligned left and right
    operands through ``extend`` or ``append``. This class maps to TA-Lib `ADD`;
    aligned history has no rolling warm-up beyond the native operation's domain
    rules.
    """

    _native_cls = _NativeMathAdd

    def append(self, left: float, right: float) -> "MathAdd":
        """Append one observation and return this indicator."""
        super().append(left, right)
        return self

    def extend(self, left: Any, right: Any) -> "MathAdd":
        """Append aligned histories and return this indicator."""
        super().extend(left, right)
        return self

    def reset(self) -> "MathAdd":
        """Reset native state and return this indicator."""
        super().reset()
        return self
