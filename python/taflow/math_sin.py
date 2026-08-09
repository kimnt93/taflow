"""Pointwise sin transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathSin


class MathSin(MathUnaryState):
    """Apply the pointwise sin operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `SIN`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathSin

    def append(self, _input: float) -> "MathSin":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathSin":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathSin":
        """Reset native state and return this indicator."""
        super().reset()
        return self
