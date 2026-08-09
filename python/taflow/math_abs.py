"""Pointwise abs transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathAbs


class MathAbs(MathUnaryState):
    """Apply the pointwise abs operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `ABS`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathAbs

    def append(self, _input: float) -> "MathAbs":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAbs":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAbs":
        """Reset native state and return this indicator."""
        super().reset()
        return self
