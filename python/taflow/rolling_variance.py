"""Persistent RollingVariance interface."""

from typing import Any

from ._unary_state import UnaryStateAdapter
from ._native import StatefulVar


class RollingVariance(UnaryStateAdapter):
    """Compute RollingVariance over a required series in native Rust state.

    ``timeperiod`` defaults to 14. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = StatefulVar

    def append(self, _input: float) -> "RollingVariance":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingVariance":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingVariance":
        """Reset native state and return this indicator."""
        super().reset()
        return self
