"""Canonical rolling maximum adapter."""
from typing import Any

from .._native import RollingMaximum
from .._unary_state import UnaryStateAdapter


class RollingMaximum(UnaryStateAdapter):
    """Compute the trailing maximum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingMaximum
        A persistent native-backed indicator adapter.
    """

    _native_cls = RollingMaximum

    def __init__(self, _input: Any, timeperiod: int = 14) -> None:
        """Create the native rolling maximum state and process ``_input``."""
        self._state = self._native_cls(timeperiod)
        self.extend(_input)

    def append(self, _input: float) -> "RollingMaximum":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingMaximum":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingMaximum":
        """Reset native state and return this indicator."""
        super().reset()
        return self
