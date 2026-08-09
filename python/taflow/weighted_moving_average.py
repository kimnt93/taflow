"""Canonical Weighted Moving Average adapter."""
from typing import Any

from ._native import StatefulWma
from ._unary_state import UnaryStateAdapter


class WeightedMovingAverage(UnaryStateAdapter):
    """Compute the linearly weighted moving average using Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    WeightedMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulWma

    def append(self, _input: float) -> "WeightedMovingAverage":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "WeightedMovingAverage":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "WeightedMovingAverage":
        """Reset native state and return this indicator."""
        super().reset()
        return self
