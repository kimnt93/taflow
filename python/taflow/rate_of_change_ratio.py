"""Canonical Rate of Change Ratio adapter."""
from typing import Any

from ._native import StatefulRocr
from ._unary_state import UnaryStateAdapter


class RateOfChangeRatio(UnaryStateAdapter):
    """Compute the rate-of-change ratio through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChangeRatio
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRocr

    def append(self, _input: float) -> "RateOfChangeRatio":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RateOfChangeRatio":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RateOfChangeRatio":
        """Reset native state and return this indicator."""
        super().reset()
        return self
