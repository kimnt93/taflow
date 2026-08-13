"""Persistent pointwise asinh transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathAsinh as _NativeMathAsinh


class MathAsinh(MathUnaryState):
    """Apply pointwise asinh in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``np.arcsinh``.
    """

    _native_cls = _NativeMathAsinh

    def append(self, _input: float) -> "MathAsinh":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAsinh":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAsinh":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
