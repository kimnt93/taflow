"""Pointwise sinh transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathSinh


class MathSinh(MathUnaryState):
    """Apply the pointwise sinh operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `SINH`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathSinh

    def append(self, _input: float) -> "MathSinh":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathSinh":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathSinh":
        """Reset native state and return this indicator."""
        super().reset()
        return self
