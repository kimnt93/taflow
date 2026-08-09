"""Pointwise tanh transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathTanh


class MathTanh(MathUnaryState):
    """Apply the pointwise tanh operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `TANH`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathTanh

    def append(self, _input: float) -> "MathTanh":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathTanh":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathTanh":
        """Reset native state and return this indicator."""
        super().reset()
        return self
