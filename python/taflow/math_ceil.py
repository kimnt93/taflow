"""Pointwise ceil transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathCeil


class MathCeil(MathUnaryState):
    """Apply the pointwise ceil operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `CEIL`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathCeil

    def append(self, _input: float) -> "MathCeil":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathCeil":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathCeil":
        """Reset native state and return this indicator."""
        super().reset()
        return self
