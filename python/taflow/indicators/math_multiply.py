"""Pointwise multiply transform."""

from typing import Any

from .._math_state import MathBinaryState
from .._native import MathMultiply as _NativeMathMultiply


class MathMultiply(MathBinaryState):
    """Apply the pointwise multiply operation in persistent Rust state.

    Construction accepts required aligned left and right operand series. This class maps to TA-Lib `MULT`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = _NativeMathMultiply

    def append(self, left: float, right: float) -> "MathMultiply":
        """Append one observation and return this indicator."""
        super().append(left, right)
        return self

    def extend(self, left: Any, right: Any) -> "MathMultiply":
        """Append aligned histories and return this indicator."""
        super().extend(left, right)
        return self

    def reset(self) -> "MathMultiply":
        """Reset native state and return this indicator."""
        super().reset()
        return self
