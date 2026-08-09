"""Canonical normalized Average True Range adapter."""
from typing import Any

from ._native import StatefulNatr
from ._ohlc_state import OhlcStateAdapter


class NormalizedAverageTrueRange(OhlcStateAdapter):
    """Compute normalized ATR through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    NormalizedAverageTrueRange
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulNatr

    def append(self, high: float, low: float, close: float) -> "NormalizedAverageTrueRange":
        """Append one observation and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "NormalizedAverageTrueRange":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "NormalizedAverageTrueRange":
        """Reset native state and return this indicator."""
        super().reset()
        return self
