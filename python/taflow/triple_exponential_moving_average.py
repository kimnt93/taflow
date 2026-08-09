"""Canonical Triple Exponential Moving Average adapter."""
from typing import Any

from ._native import StatefulTema
from ._unary_state import UnaryStateAdapter


class TripleExponentialMovingAverage(UnaryStateAdapter):
    """Compute TEMA through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    TripleExponentialMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulTema

    def append(self, _input: float) -> "TripleExponentialMovingAverage":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "TripleExponentialMovingAverage":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "TripleExponentialMovingAverage":
        """Reset native state and return this indicator."""
        super().reset()
        return self
