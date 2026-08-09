"""Canonical Triangular Moving Average adapter."""
from typing import Any

from ._native import StatefulTrima
from ._unary_state import UnaryStateAdapter


class TriangularMovingAverage(UnaryStateAdapter):
    """Compute the triangular moving average through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    TriangularMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulTrima

    def append(self, _input: float) -> "TriangularMovingAverage":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "TriangularMovingAverage":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "TriangularMovingAverage":
        """Reset native state and return this indicator."""
        super().reset()
        return self
