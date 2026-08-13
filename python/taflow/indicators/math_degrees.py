"""Persistent pointwise degrees transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathDegrees as _NativeMathDegrees


class MathDegrees(MathUnaryState):
    """Apply pointwise degrees in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``np.degrees``.
    """

    _native_cls = _NativeMathDegrees

    def append(self, _input: float) -> "MathDegrees":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathDegrees":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathDegrees":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
