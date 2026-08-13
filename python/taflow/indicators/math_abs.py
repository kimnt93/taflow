"""Persistent pointwise abs transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathAbs as _NativeMathAbs


class MathAbs(MathUnaryState):
    """Apply pointwise abs in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``np.abs``.
    """

    _native_cls = _NativeMathAbs

    def append(self, _input: float) -> "MathAbs":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAbs":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAbs":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
