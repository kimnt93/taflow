"""Pointwise degrees transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathDegrees


class MathDegrees(MathUnaryState):
    """Apply the pointwise degrees operation in persistent Rust state.

    Construction accepts a required input series. This class maps to the equivalent Polars expression; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathDegrees

    def append(self, _input: float) -> "MathDegrees":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathDegrees":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathDegrees":
        """Reset native state and return this indicator."""
        super().reset()
        return self
