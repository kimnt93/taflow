"""Persistent RollingLinearRegressionSlope interface."""

from typing import Any

from ._unary_state import UnaryStateAdapter
from ._native import StatefulLinearregSlope


class RollingLinearRegressionSlope(UnaryStateAdapter):
    """Compute RollingLinearRegressionSlope over a required series in native Rust state.

    ``timeperiod`` defaults to 14. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = StatefulLinearregSlope

    def append(self, _input: float) -> "RollingLinearRegressionSlope":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingLinearRegressionSlope":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingLinearRegressionSlope":
        """Reset native state and return this indicator."""
        super().reset()
        return self
