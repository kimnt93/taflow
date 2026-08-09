"""Persistent RollingAverageDeviation interface."""

from typing import Any

from ._unary_state import UnaryStateAdapter
from ._native import StatefulAvgdev


class RollingAverageDeviation(UnaryStateAdapter):
    """Compute RollingAverageDeviation over a required series in native Rust state.

    ``timeperiod`` defaults to 14. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = StatefulAvgdev

    def append(self, _input: float) -> "RollingAverageDeviation":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingAverageDeviation":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingAverageDeviation":
        """Reset native state and return this indicator."""
        super().reset()
        return self
