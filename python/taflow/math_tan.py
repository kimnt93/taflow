"""Pointwise tan transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathTan


class MathTan(MathUnaryState):
    """Apply the pointwise tan operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `TAN`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathTan

    def append(self, _input: float) -> "MathTan":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathTan":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathTan":
        """Reset native state and return this indicator."""
        super().reset()
        return self
