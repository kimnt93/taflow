"""Pointwise log1p transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathLog1p


class MathLog1p(MathUnaryState):
    """Apply the pointwise log1p operation in persistent Rust state.

    Construction accepts a required input series. This class maps to the equivalent Polars expression; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathLog1p

    def append(self, _input: float) -> "MathLog1p":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathLog1p":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathLog1p":
        """Reset native state and return this indicator."""
        super().reset()
        return self
