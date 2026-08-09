"""Pointwise floor transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathFloor


class MathFloor(MathUnaryState):
    """Apply the pointwise floor operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `FLOOR`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathFloor

    def append(self, _input: float) -> "MathFloor":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathFloor":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathFloor":
        """Reset native state and return this indicator."""
        super().reset()
        return self
