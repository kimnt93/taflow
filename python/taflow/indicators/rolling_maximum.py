"""Canonical rolling maximum adapter."""
from typing import Any

from .._native import RollingMaximum
from .._unary_state import UnaryStateAdapter


class RollingMaximum(UnaryStateAdapter):
    """Compute the trailing maximum through the native Rust state

    Parameters
    ----------
    Construct with configuration values only; supply input series through ``extend``.

    Returns
    -------
    RollingMaximum
        A persistent native-backed indicator adapter.
    """

    _native_cls = RollingMaximum

    def __init__(self, timeperiod: int = 14) -> None:
        """Initialize an empty configured native state.
        """
        self._state = self._native_cls(timeperiod)

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
