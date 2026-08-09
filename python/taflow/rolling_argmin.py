"""Canonical rolling argmin adapter."""
from typing import Any

from ._native import StatefulMinindex
from ._unary_state import UnaryStateAdapter


class RollingArgmin(UnaryStateAdapter):
    """Compute trailing minimum indices through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingArgmin
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMinindex

    def __init__(self, _input: Any, timeperiod: int = 14) -> None:
        """Create the native minimum-index state and process ``_input``."""
        self._state = self._native_cls(timeperiod)
        self.extend(_input)

    def append(self, _input: float) -> "RollingArgmin":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingArgmin":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingArgmin":
        """Reset native state and return this indicator."""
        super().reset()
        return self
