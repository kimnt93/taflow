"""Canonical rolling minimum adapter."""
from typing import Any

from .._native import RollingMinimum
from .._unary_state import UnaryStateAdapter


class RollingMinimum(UnaryStateAdapter):
    """Compute the trailing minimum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingMinimum
        A persistent native-backed indicator adapter.
    """

    _native_cls = RollingMinimum

    def __init__(self, _input: Any, timeperiod: int = 14) -> None:
        """Create the native rolling minimum state and process ``_input``."""
        self._state = self._native_cls(timeperiod)
        self.extend(_input)

    def append(self, _input: float) -> "RollingMinimum":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingMinimum":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingMinimum":
        """Reset native state and return this indicator."""
        super().reset()
        return self
