"""Persistent pointwise exp transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathExp as _NativeMathExp


class MathExp(MathUnaryState):
    """Apply pointwise exp in persistent Rust state.

    Parameters:
        _input: Required chronological values. Pass an empty series for a fresh
            streaming state.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``talib.EXP``.
    """

    _native_cls = _NativeMathExp

    def append(self, _input: float) -> "MathExp":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathExp":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathExp":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
