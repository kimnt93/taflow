"""Canonical Double Exponential Moving Average adapter."""
from typing import Any

from ._native import StatefulDema
from ._unary_state import UnaryStateAdapter


class DoubleExponentialMovingAverage(UnaryStateAdapter):
    """Compute DEMA through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    DoubleExponentialMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulDema

    def append(self, _input: float) -> "DoubleExponentialMovingAverage":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "DoubleExponentialMovingAverage":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "DoubleExponentialMovingAverage":
        """Reset native state and return this indicator."""
        super().reset()
        return self
