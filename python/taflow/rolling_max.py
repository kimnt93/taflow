"""Canonical rolling maximum adapter."""
from typing import Any

from ._native import StatefulMax
from ._unary_state import UnaryStateAdapter


class RollingMax(UnaryStateAdapter):
    """Compute the trailing maximum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingMax
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMax

    def __init__(self, _input: Any, timeperiod: int = 14) -> None:
        """Create the native rolling maximum state and process ``_input``."""
        self._state = self._native_cls(timeperiod)
        self.extend(_input)

    def append(self, _input: float) -> "RollingMax":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingMax":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingMax":
        """Reset native state and return this indicator."""
        super().reset()
        return self
