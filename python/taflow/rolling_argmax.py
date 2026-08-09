"""Canonical rolling argmax adapter."""
from typing import Any

from ._native import StatefulMaxindex
from ._unary_state import UnaryStateAdapter


class RollingArgmax(UnaryStateAdapter):
    """Compute trailing maximum indices through the native Rust state.

    Parameters
    ----------
    timeperiod : int
        Number of observations in each trailing window.
    input_values : array-like
        Initial chronological values processed by the native state.

    Returns
    -------
    RollingArgmax
        A persistent native-backed rolling operator.
    """

    _native_cls = StatefulMaxindex

    def append(self, _input: float) -> "RollingArgmax":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingArgmax":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingArgmax":
        """Reset native state and return this indicator."""
        super().reset()
        return self
