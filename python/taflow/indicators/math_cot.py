"""Persistent pointwise cot transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathCot as _NativeMathCot


class MathCot(MathUnaryState):
    """Apply pointwise cot in persistent Rust state.

    Constructing the class creates a fresh empty state. Supply chronological
    values through ``extend`` or ``append``.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``lambda values: 1.0 / np.tan(values)``.
    """

    _native_cls = _NativeMathCot

    def append(self, _input: float) -> "MathCot":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathCot":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathCot":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
