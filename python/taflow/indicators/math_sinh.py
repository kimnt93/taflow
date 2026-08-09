"""Persistent pointwise sinh transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathSinh as _NativeMathSinh


class MathSinh(MathUnaryState):
    """Apply pointwise sinh in persistent Rust state.

    Parameters:
        _input: Required chronological values. Pass an empty series for a fresh
            streaming state.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``talib.SINH``.
    """

    _native_cls = _NativeMathSinh

    def append(self, _input: float) -> "MathSinh":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathSinh":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathSinh":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
