"""Canonical fractional Rate of Change adapter."""
from typing import Any

from ._native import StatefulRocp
from ._unary_state import UnaryStateAdapter


class RateOfChangePercent(UnaryStateAdapter):
    """Compute fractional rate of change through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChangePercent
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRocp

    def append(self, _input: float) -> "RateOfChangePercent":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RateOfChangePercent":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RateOfChangePercent":
        """Reset native state and return this indicator."""
        super().reset()
        return self
