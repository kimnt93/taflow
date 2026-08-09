"""Canonical Simple Moving Average adapter."""
from typing import Any

from ._native import StatefulSma
from ._unary_state import UnaryStateAdapter


class SimpleMovingAverage(UnaryStateAdapter):
    """Compute the simple moving average using the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    SimpleMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulSma

    def append(self, _input: float) -> "SimpleMovingAverage":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "SimpleMovingAverage":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "SimpleMovingAverage":
        """Reset native state and return this indicator."""
        super().reset()
        return self
