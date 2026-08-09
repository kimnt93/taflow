"""Pointwise sqrt transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathSqrt


class MathSqrt(MathUnaryState):
    """Apply the pointwise sqrt operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `SQRT`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathSqrt

    def append(self, _input: float) -> "MathSqrt":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathSqrt":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathSqrt":
        """Reset native state and return this indicator."""
        super().reset()
        return self
