"""Pointwise ln transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathLn


class MathLn(MathUnaryState):
    """Apply the pointwise ln operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `LN`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathLn

    def append(self, _input: float) -> "MathLn":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathLn":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathLn":
        """Reset native state and return this indicator."""
        super().reset()
        return self
