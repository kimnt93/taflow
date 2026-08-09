"""Canonical Rate of Change adapter."""
from typing import Any

from ._native import StatefulRoc
from ._unary_state import UnaryStateAdapter


class RateOfChange(UnaryStateAdapter):
    """Compute percentage rate of change through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChange
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRoc

    def append(self, _input: float) -> "RateOfChange":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RateOfChange":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RateOfChange":
        """Reset native state and return this indicator."""
        super().reset()
        return self
