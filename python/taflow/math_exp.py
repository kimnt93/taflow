"""Pointwise exp transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import StatefulMathExp


class MathExp(MathUnaryState):
    """Apply the pointwise exp operation in persistent Rust state.

    Construction accepts a required input series. This class maps to TA-Lib `EXP`; aligned
    history has no rolling warm-up beyond the native operation's domain rules.
    """

    _native_cls = StatefulMathExp

    def append(self, _input: float) -> "MathExp":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathExp":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathExp":
        """Reset native state and return this indicator."""
        super().reset()
        return self
