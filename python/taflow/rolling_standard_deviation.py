"""Persistent RollingStandardDeviation interface."""

from typing import Any

from ._unary_state import UnaryStateAdapter
from ._native import StatefulStddev


class RollingStandardDeviation(UnaryStateAdapter):
    """Compute RollingStandardDeviation over a required series in native Rust state.

    ``timeperiod`` defaults to 14. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = StatefulStddev

    def append(self, _input: float) -> "RollingStandardDeviation":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingStandardDeviation":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingStandardDeviation":
        """Reset native state and return this indicator."""
        super().reset()
        return self
