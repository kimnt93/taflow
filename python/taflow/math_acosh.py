"""Persistent pointwise acosh transform."""

from typing import Any

from ._math_state import MathUnaryState
from ._native import MathAcosh as _NativeMathAcosh


class MathAcosh(MathUnaryState):
    """Apply pointwise acosh in persistent Rust state.

    Parameters:
        _input: Required chronological values. Pass an empty series for a fresh
            streaming state.

    The output is a same-length ``float64`` array with no rolling warm-up.
    Domain behavior follows IEEE 754. The independent correctness oracle is
    ``np.arccosh``.
    """

    _native_cls = _NativeMathAcosh

    def append(self, _input: float) -> "MathAcosh":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "MathAcosh":
        """Append chronological values and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "MathAcosh":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
