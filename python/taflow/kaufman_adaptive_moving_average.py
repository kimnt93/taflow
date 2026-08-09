"""Canonical Kaufman Adaptive Moving Average adapter."""
from typing import Any

from ._native import StatefulKama
from ._unary_state import UnaryStateAdapter


class KaufmanAdaptiveMovingAverage(UnaryStateAdapter):
    """Compute KAMA through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    KaufmanAdaptiveMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulKama

    def append(self, _input: float) -> "KaufmanAdaptiveMovingAverage":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "KaufmanAdaptiveMovingAverage":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "KaufmanAdaptiveMovingAverage":
        """Reset native state and return this indicator."""
        super().reset()
        return self
