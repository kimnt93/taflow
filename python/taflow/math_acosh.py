"""Pointwise acosh transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathAcosh


class MathAcosh(MathUnaryState):
    """Apply the pointwise acosh operation in persistent Rust state.

    Construction accepts a required input series. This class maps to the equivalent Polars expression; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathAcosh

    def append(self, _input: float) -> "MathAcosh":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAcosh":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAcosh":
        """Reset native state and return this indicator."""
        super().reset()
        return self
