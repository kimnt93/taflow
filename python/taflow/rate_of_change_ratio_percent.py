"""Canonical percent Rate of Change Ratio adapter."""
from typing import Any

from ._native import StatefulRocr100
from ._unary_state import UnaryStateAdapter


class RateOfChangeRatioPercent(UnaryStateAdapter):
    """Compute the 100-scaled rate-of-change ratio through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChangeRatioPercent
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRocr100

    def append(self, _input: float) -> "RateOfChangeRatioPercent":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RateOfChangeRatioPercent":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RateOfChangeRatioPercent":
        """Reset native state and return this indicator."""
        super().reset()
        return self
