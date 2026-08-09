"""Pointwise log10 transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathLog10


class MathLog10(MathUnaryState):
    """Apply the pointwise log10 operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `LOG10`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathLog10

    def append(self, _input: float) -> "MathLog10":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathLog10":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathLog10":
        """Reset native state and return this indicator."""
        super().reset()
        return self
