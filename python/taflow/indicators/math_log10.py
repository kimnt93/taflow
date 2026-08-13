"""Persistent pointwise log10 transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathLog10 as _NativeMathLog10


class MathLog10(MathUnaryState):
    """Apply pointwise log10 in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``talib.LOG10``.
    """

    _native_cls = _NativeMathLog10

    def append(self, _input: float) -> "MathLog10":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathLog10":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathLog10":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
