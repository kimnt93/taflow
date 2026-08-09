"""Pointwise asin transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathAsin


class MathAsin(MathUnaryState):
    """Apply the pointwise asin operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `ASIN`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathAsin

    def append(self, _input: float) -> "MathAsin":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAsin":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAsin":
        """Reset native state and return this indicator."""
        super().reset()
        return self
