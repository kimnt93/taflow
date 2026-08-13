"""Persistent pointwise log1p transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathLog1p as _NativeMathLog1p


class MathLog1p(MathUnaryState):
    """Apply pointwise log1p in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``np.log1p``.
    """

    _native_cls = _NativeMathLog1p

    def append(self, _input: float) -> "MathLog1p":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathLog1p":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathLog1p":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
