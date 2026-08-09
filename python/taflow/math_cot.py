"""Pointwise cot transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathCot


class MathCot(MathUnaryState):
    """Apply the pointwise cot operation in persistent Rust state.

    Construction accepts a required input series. This class maps to the equivalent Polars expression; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathCot

    def append(self, _input: float) -> "MathCot":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathCot":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathCot":
        """Reset native state and return this indicator."""
        super().reset()
        return self
