"""Pointwise acos transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathAcos


class MathAcos(MathUnaryState):
    """Apply the pointwise acos operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `ACOS`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathAcos

    def append(self, _input: float) -> "MathAcos":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAcos":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAcos":
        """Reset native state and return this indicator."""
        super().reset()
        return self
