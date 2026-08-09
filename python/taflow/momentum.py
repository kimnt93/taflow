"""Canonical Momentum adapter."""
from typing import Any

from ._native import StatefulMom
from ._unary_state import UnaryStateAdapter


class Momentum(UnaryStateAdapter):
    """Compute price momentum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    Momentum
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMom

    def append(self, _input: float) -> "Momentum":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "Momentum":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "Momentum":
        """Reset native state and return this indicator."""
        super().reset()
        return self
