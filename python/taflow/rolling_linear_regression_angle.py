"""Persistent RollingLinearRegressionAngle interface."""

from typing import Any

from ._unary_state import UnaryStateAdapter
from ._native import StatefulLinearregAngle


class RollingLinearRegressionAngle(UnaryStateAdapter):
    """Compute RollingLinearRegressionAngle over a required series in native Rust state.

    ``timeperiod`` defaults to 14. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = StatefulLinearregAngle

    def append(self, _input: float) -> "RollingLinearRegressionAngle":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingLinearRegressionAngle":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingLinearRegressionAngle":
        """Reset native state and return this indicator."""
        super().reset()
        return self
