"""Persistent RollingLinearRegression interface."""

from typing import Any

from .._unary_state import UnaryStateAdapter
from .._native import RollingLinearRegression as _NativeRollingLinearRegression


class RollingLinearRegression(UnaryStateAdapter):
    """Compute RollingLinearRegression over a required series in native Rust state.

    ``timeperiod`` defaults to 14. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = _NativeRollingLinearRegression

    def append(self, _input: float) -> "RollingLinearRegression":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingLinearRegression":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingLinearRegression":
        """Reset native state and return this indicator."""
        super().reset()
        return self
