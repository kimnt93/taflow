"""Persistent pointwise cos transform."""

from typing import Any

from .._math_state import MathUnaryState
from .._native import MathCos as _NativeMathCos


class MathCos(MathUnaryState):
    """Apply pointwise cos in persistent Rust state.

    Parameters:
        _input: Required chronological values. Pass an empty series for a fresh
            streaming state.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``talib.COS``.
    """

    _native_cls = _NativeMathCos

    def append(self, _input: float) -> "MathCos":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathCos":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathCos":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
