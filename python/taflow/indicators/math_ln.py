"""Persistent pointwise ln transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathLn as _NativeMathLn


class MathLn(MathUnaryState):
    """Apply pointwise ln in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``talib.LN``.
    """

    _native_cls = _NativeMathLn

    def append(self, _input: float) -> "MathLn":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathLn":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathLn":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
