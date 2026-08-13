"""Canonical rolling argmin adapter."""
from typing import Any

from .._native import RollingMinimumIndex
from .._unary_state import UnaryStateAdapter


class RollingMinimumIndex(UnaryStateAdapter):
    """Compute trailing minimum indices through the native Rust state

    Parameters
    ----------
    Construct with configuration values only; supply input series through ``extend``.

    Returns
    -------
    RollingMinimumIndex
        A persistent native-backed indicator adapter.
    """

    _native_cls = RollingMinimumIndex

    def __init__(self, timeperiod: int = 14) -> None:
        """Initialize an empty configured native state.
        """
        self._state = self._native_cls(timeperiod)

    def append(self, _input: float) -> "RollingMinimumIndex":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingMinimumIndex":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingMinimumIndex":
        """Reset native state and return this indicator."""
        super().reset()
        return self
