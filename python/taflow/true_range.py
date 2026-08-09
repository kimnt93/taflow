"""Canonical True Range adapter."""
from typing import Any

from ._native import StatefulTrange
from ._ohlc_state import OhlcStateAdapter


class TrueRange(OhlcStateAdapter):
    """Compute true range from high, low, and previous close values

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    TrueRange
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulTrange
    _period_required = False

    def append(self, high: float, low: float, close: float) -> "TrueRange":
        """Append one observation and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "TrueRange":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "TrueRange":
        """Reset native state and return this indicator."""
        super().reset()
        return self
