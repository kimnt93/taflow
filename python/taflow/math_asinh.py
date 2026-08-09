"""Pointwise asinh transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathAsinh


class MathAsinh(MathUnaryState):
    """Apply the pointwise asinh operation in persistent Rust state.

    Construction accepts a required input series. This class maps to the equivalent Polars expression; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathAsinh

    def append(self, _input: float) -> "MathAsinh":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAsinh":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAsinh":
        """Reset native state and return this indicator."""
        super().reset()
        return self
