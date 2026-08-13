"""Canonical rolling argmax adapter."""
from typing import Any

from .._native import RollingMaximumIndex
from .._unary_state import UnaryStateAdapter


class RollingMaximumIndex(UnaryStateAdapter):
    """Compute trailing maximum indices through the native Rust state.

    Parameters
    ----------
    timeperiod : int
        Number of observations in each trailing window.
    input_values : array-like
        Chronological values processed by the native state.

    Returns
    -------
    RollingMaximumIndex
        A persistent native-backed rolling operator.
    """

    _native_cls = RollingMaximumIndex

    def __init__(self, timeperiod: int = 14) -> None:
        """Initialize an empty configured native state.
        """
        self._state = self._native_cls(timeperiod)

    def append(self, _input: float) -> "RollingMaximumIndex":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingMaximumIndex":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingMaximumIndex":
        """Reset native state and return this indicator."""
        super().reset()
        return self
