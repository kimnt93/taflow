"""Persistent pointwise cbrt transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathCbrt as _NativeMathCbrt


class MathCbrt(MathUnaryState):
    """Apply pointwise cbrt in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``np.cbrt``.
    """

    _native_cls = _NativeMathCbrt

    def append(self, _input: float) -> "MathCbrt":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathCbrt":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathCbrt":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
