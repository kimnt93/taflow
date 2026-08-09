"""Pointwise cosh transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathCosh


class MathCosh(MathUnaryState):
    """Apply the pointwise cosh operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `COSH`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathCosh

    def append(self, _input: float) -> "MathCosh":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathCosh":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathCosh":
        """Reset native state and return this indicator."""
        super().reset()
        return self
