"""Canonical rolling sum adapter."""
from typing import Any

from ._native import StatefulSum
from ._unary_state import UnaryStateAdapter


class RollingSum(UnaryStateAdapter):
    """Compute the trailing sum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingSum
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulSum

    def append(self, _input: float) -> "RollingSum":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingSum":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingSum":
        """Reset native state and return this indicator."""
        super().reset()
        return self
