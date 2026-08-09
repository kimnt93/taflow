"""Pointwise atan transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathAtan


class MathAtan(MathUnaryState):
    """Apply the pointwise atan operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `ATAN`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathAtan

    def append(self, _input: float) -> "MathAtan":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAtan":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAtan":
        """Reset native state and return this indicator."""
        super().reset()
        return self
